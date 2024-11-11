use anyhow::{Context, Result};
use scraper::{Html, Selector};
use zip;

struct Cert {
    name: String,
    url: String,
}

struct CertConfig {
    base_url: &'static str,
    certs_url: &'static str,
    root_cert_url: &'static str,
    output_dir: &'static str,
}

async fn download_certs(config: CertConfig) -> Result<()> {
    log::info!(
        "Starting download of certificates from {}",
        config.certs_url
    );

    // Setup directory
    log::info!("Cleaning up existing certificates directory");
    std::fs::remove_dir(config.output_dir).ok();
    std::fs::create_dir_all(config.output_dir).ok();

    let certs_to_download = fetch_cert_list(&config).await?;
    download_all_certs(certs_to_download, config.output_dir).await?;

    download_root_cert(&config).await?;

    log::info!("Successfully downloaded all certificates");
    Ok(())
}

async fn fetch_cert_list(config: &CertConfig) -> Result<Vec<Cert>> {
    log::info!(
        "Fetching certificate list from {}{}",
        config.base_url,
        config.certs_url
    );
    let response = reqwest::get(format!("{}{}", config.base_url, config.certs_url))
        .await
        .context("Failed to get response from certs URI")?;
    let body = response
        .text()
        .await
        .context("Failed to get body of response")?;
    let parser = Html::parse_document(&body);

    let mut certs_to_download = Vec::new();

    log::info!("Looking for certificate download links");

    // Find all links with the specific title
    let link_selector = &Selector::parse("a[title='Download certificate file']").unwrap();
    let links = parser.select(link_selector);

    for link in links {
        let name = link.text().collect::<String>();
        let href = link
            .value()
            .attr("href")
            .expect("Failed to get href of link");
        certs_to_download.push(Cert {
            name: format!("{}.bin", name.trim().to_string()),
            url: format!("{}/{href}", config.base_url),
        });
    }

    // Duplicates happen when the same cert name is available with different validity dates
    // So the older certificates are given a suffix of _1, _2, etc.
    for i in 0..certs_to_download.len() {
        let mut count = 1;
        for j in 0..i {
            if certs_to_download[j].name == certs_to_download[i].name {
                count += 1;
            }
        }
        if count > 1 {
            let name = certs_to_download[i].name.replace(".bin", "");
            certs_to_download[i].name = format!("{}_{}.bin", name, count);
        }
    }

    log::info!("Found {} certificates to download", certs_to_download.len());

    Ok(certs_to_download)
}

async fn download_root_cert(config: &CertConfig) -> Result<()> {
    let root_cert_url = format!("{}{}", config.base_url, config.root_cert_url);
    let response = reqwest::get(root_cert_url).await?;
    let content = response.bytes().await?;

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(content))?;
    let root_dir = format!("{}/root", config.output_dir);
    std::fs::create_dir_all(&root_dir)?;
    archive.extract(&root_dir)?;
    for i in 0..archive.len() {
        let file = archive
            .by_index(i)
            .context("Failed to get file from archive")?;
        log::info!("Extracted {} from root certificate archive", file.name());
    }

    Ok(())
}

async fn download_all_certs(certs: Vec<Cert>, output_dir: &str) -> Result<()> {
    log::info!("Starting download of {} certificates", certs.len());
    let download_tasks = certs.into_iter().map(|cert| {
        let output_dir = output_dir.to_string();
        tokio::spawn(async move { download_single_cert(cert, &output_dir).await })
    });

    futures::future::join_all(download_tasks)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}

async fn download_single_cert(cert: Cert, output_dir: &str) -> Result<()> {
    let max_attempts = 3;
    let mut last_error: Option<anyhow::Error> = None;

    for attempt in 1..=max_attempts {
        let response = reqwest::get(&cert.url).await?;
        let bytes_result = response.bytes().await;
        match bytes_result {
            Ok(content) => {
                let output_path = format!("{}/{}", output_dir, cert.name);
                let error_msg = format!("Failed to save certificate {}", cert.name);
                std::fs::write(&output_path, &content).context(error_msg)?;

                log::info!(
                    "Successfully downloaded and saved certificate: {}",
                    cert.name
                );
                return Ok(());
            }
            Err(e) => {
                log::warn!(
                    "Download attempt {}/{} failed: {}",
                    attempt,
                    max_attempts,
                    e
                );
                last_error = Some(e.into());
            }
        }
    }

    Err(last_error
        .unwrap_or_else(|| anyhow::anyhow!("Failed to download {}", cert.url))
        .into())
}

pub async fn download_gen1_certs() -> Result<()> {
    download_certs(CertConfig {
        base_url: "https://dtc.jrc.ec.europa.eu",
        certs_url: "/dtc_public_key_certificates_dt.php.html",
        root_cert_url: "/erca_of_doc/EC_PK.zip",
        output_dir: "certs/pks1",
    })
    .await
}

pub async fn download_gen2_certs() -> Result<()> {
    download_certs(CertConfig {
        base_url: "https://dtc.jrc.ec.europa.eu",
        certs_url: "/dtc_public_key_certificates_st.php.html",
        root_cert_url: "/ERCA_Gen2_Root_Certificate.zip",
        output_dir: "certs/pks2",
    })
    .await
}
