use gio::prelude::*; // Import the necessary traits
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label};

fn main() {
    let app = Application::new(Some("com.example.quickemu-installer"), Default::default())
        .expect("Failed to initialize GTK.");

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Quickemu Installer");
        window.set_default_size(350, 70);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let label = Label::new(Some("Click the button to install and run Quickemu"));
        let button = Button::with_label("Install and Run");

        button.connect_clicked(|_| {
            if is_debian_based() {
                install_quickemu();
                setup_and_run_vm();
            } else {
                eprintln!("This script only supports Debian-based systems.");
            }
        });

        vbox.append(&label);
        vbox.append(&button);
        window.set_child(Some(&vbox));
        window.show();
    });

    app.run();
}

fn is_debian_based() -> bool {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("cat /etc/os-release | grep 'ID_LIKE=debian'")
        .output()
        .expect("Failed to execute command");

    output.status.success()
}

fn install_quickemu() {
    let url = "https://github.com/quickemu-project/quickemu/releases/download/4.9.6/quickemu_4.9.6-1_all.deb";
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("wget {} -O /tmp/quickemu.deb", url))
        .output()
        .expect("Failed to download Quickemu");

    if output.status.success() {
        std::process::Command::new("sh")
            .arg("-c")
            .arg("sudo dpkg -i /tmp/quickemu.deb")
            .output()
            .expect("Failed to install Quickemu");
    } else {
        eprintln!("Failed to download Quickemu");
    }
}

fn setup_and_run_vm() {
    let sandbox_dir = format!(
        "{}/.local/share/linux-sandbox",
        std::env::var("HOME").unwrap()
    );
    if !std::path::Path::new(&sandbox_dir).exists() {
        std::fs::create_dir_all(&sandbox_dir).expect("Failed to create sandbox directory");
    }

    let vm_conf = format!("{}/linuxmint-21.3-cinnamon.conf", sandbox_dir);
    if !std::path::Path::new(&vm_conf).exists() {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(format!(
                "cd {} && quickget linuxmint 21.3 cinnamon",
                sandbox_dir
            ))
            .output()
            .expect("Failed to get Linux Mint VM");
    }

    std::process::Command::new("sh")
        .arg("-c")
        .arg(format!(
            "cd {} && quickemu --vm linuxmint-21.3-cinnamon.conf --viewer none --display spice --access local --spice-port 5999 --public-dir ~/Public --status-quo",
            sandbox_dir
        ))
        .output()
        .expect("Failed to run VM");

    std::process::Command::new("sh")
        .arg("-c")
        .arg("remote-viewer spice://localhost:5999")
        .output()
        .expect("Failed to open remote viewer");
}
