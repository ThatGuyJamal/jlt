pub const ARCH_SETUP_DOCKER: &str = r#"
#!/bin/bash

FILE_NAME=docker-desktop-x86_64.pkg.tar.zst
TEMP_DIR=$(mktemp -d)

sudo -S pacman -S gnome-terminal # Required for docker
sudo -S  pacman -S docker -y

# Add the current user to the docker group using the who command
sudo usermod -aG docker $(whoami)
su $(whoami)

# Download Docker Desktop package to the temporary directory
wget -P $TEMP_DIR https://desktop.docker.com/linux/main/amd64/$FILE_NAME

# Extract the package in the temporary directory
tar --use-compress-program=unzstd -xvf $TEMP_DIR/$FILE_NAME -C $TEMP_DIR

# Install Docker Desktop
sudo -S  pacman -U --noconfirm $TEMP_DIR/$FILE_NAME

# Clean up
rm -rf $TEMP_DIR
"#;

pub const DEB_SETUP_DOCKER: &str = r#"
#!/bin/bash

FILE_NAME=docker-desktop-<version>-amd64.deb
TEMP_DIR=$(mktemp -d)

# Update the package list
sudo -S  apt-get update

# Install required packages
sudo -S  apt-get install -y gnome-terminal wget apt-transport-https ca-certificates curl software-properties-common

# Install Docker
sudo -S  apt-get install -y docker.io

# Add the current user to the docker group using the who command
sudo -S  usermod -aG docker $(whoami)
su $(whoami)

# Download Docker Desktop package to the temporary directory
wget -P $TEMP_DIR https://desktop.docker.com/linux/main/amd64/$FILE_NAME

# Install Docker Desktop using dpkg
sudo -S  dpkg -i $TEMP_DIR/$FILE_NAME

# Resolve any missing dependencies
sudo -S  apt-get install -f -y

# Clean up
rm -rf $TEMP_DIR
"#;
