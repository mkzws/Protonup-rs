use clap::Parser;

use inquire::Select;

use std::fmt;

use libprotonup::apps::App;

mod download;
mod file_path;
mod helper_menus;
mod manage_apps;

use manage_apps::manage_apps_routine;

#[derive(Debug, Parser)]
struct Opt {
    /// Skip Menu, auto detect apps and download using default parameters
    #[arg(short, long)]
    quick_download: bool,

    /// Force install for existing apps during quick downloads
    #[arg(short, long)]
    force: bool,
}

#[derive(Debug, Copy, Clone)]
#[allow(clippy::upper_case_acronyms)]
enum InitialMenu {
    QuickUpdate,
    DownloadForSteam,
    DownloadForLutris,
    DownloadIntoCustomLocation,
    ManageExistingInstallations,
}

impl InitialMenu {
    // could be generated by macro
    const VARIANTS: &'static [InitialMenu] = &[
        Self::QuickUpdate,
        Self::DownloadForSteam,
        Self::DownloadForLutris,
        Self::DownloadIntoCustomLocation,
        Self::ManageExistingInstallations,
    ];
}

impl fmt::Display for InitialMenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::QuickUpdate => write!(
                f,
                "Quick Update (detect apps and update compatibility tools)"
            ),
            Self::DownloadForSteam => write!(f, "Download compatibility tools for Steam"),
            Self::DownloadForLutris => write!(f, "Download compatibility tools for Lutris"),
            Self::DownloadIntoCustomLocation => {
                write!(f, "Download compatibility tools into custom location")
            }
            Self::ManageExistingInstallations => write!(f, "Manage Existing Installations"),
        }
    }
}

#[tokio::main]
async fn main() {
    // run quick downloads and skip InitialMenu
    let Opt {
        quick_download,
        force,
    } = Opt::parse();
    let releases = if quick_download {
        download::run_quick_downloads(force).await
    } else {
        let answer: InitialMenu = Select::new(
            "ProtonUp Menu: Choose your action:",
            InitialMenu::VARIANTS.to_vec(),
        )
        .with_page_size(10)
        .prompt()
        .unwrap_or_else(|_| std::process::exit(0));

        // Set parameters based on users choice
        match answer {
            InitialMenu::QuickUpdate => download::run_quick_downloads(force).await,
            InitialMenu::DownloadForSteam => {
                download::download_to_selected_app(Some(App::Steam)).await
            }
            InitialMenu::DownloadForLutris => {
                download::download_to_selected_app(Some(App::Lutris)).await
            }
            InitialMenu::DownloadIntoCustomLocation => {
                download::download_to_selected_app(None).await
            }
            InitialMenu::ManageExistingInstallations => {
                manage_apps_routine().await;
                Ok(vec![])
            }
        }
    };

    if let Ok(releases) = releases {
        for release in releases {
            println!("Installed {}", release.tag_name);
        }
    }
}
