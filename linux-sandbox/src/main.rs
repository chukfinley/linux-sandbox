use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label, Orientation};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let app = Application::builder()
        .application_id("com.example.quickemu-installer")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Quickemu Installer")
            .default_width(350)
            .default_height(70)
            .build();

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let label = Label::builder()
            .label("Click the button to install and run Quickemu")
            .build();
        let button = Button::builder().label("Install and Run").build();

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
    let output = Command::new("sh")
        .arg("-c")
        .arg("cat /etc/os-release | grep 'ID_LIKE=debian'")
        .output()
        .expect("Failed to execute command");

    output.status.success()
}

fn install_quickemu() {
    let url = "https://github.com/quickemu-project/quickemu/releases/download/4.9.6/quickemu_4.9.6-1_all.deb";
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("wget {} -O /tmp/quickemu.deb", url))
        .output()
        .expect("Failed to download Quickemu");

    if output.status.success() {
        Command::new("sh")
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
    if !Path::new(&sandbox_dir).exists() {
        fs::create_dir_all(&sandbox_dir).expect("Failed to create sandbox directory");
    }

    let vm_conf = format!("{}/linuxmint-21.3-cinnamon.conf", sandbox_dir);
    if !Path::new(&vm_conf).exists() {
        Command::new("sh")
            .arg("-c")
            .arg(format!(
                "cd {} && quickget linuxmint 21.3 cinnamon",
                sandbox_dir
            ))
            .output()
            .expect("Failed to get Linux Mint VM");
    }

    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "cd {} && quickemu --vm linuxmint-21.3-cinnamon.conf --viewer none --display spice --access local --spice-port 5999 --public-dir ~/Public --status-quo",
            sandbox_dir
        ))
        .output()
        .expect("Failed to run VM");

    Command::new("sh")
        .arg("-c")
        .arg("remote-viewer spice://localhost:5999")
        .output()
        .expect("Failed to open remote viewer");
}
