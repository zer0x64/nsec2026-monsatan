# NorthSec CTF 2026 - Monsatan Track

## Introduction

This repository contains most of the challenges for the Monsatan track of the NorthSec CTF 2026.

This repository was generated using the [`ctf-script` tool](https://github.com/nsec/ctf-script). To deploy a track, install the script first:
```bash
uv tool install git+https://github.com/nsec/ctf-script.git
# OR
pipx install git+https://github.com/nsec/ctf-script.git
# OR
pip install git+https://github.com/nsec/ctf-script.git
```

For reference, this repo has been tested using the script with the following commit hash: `1f974c16822952d91dd37362bac8cebd0cff8bfa`

## Metatrack structure

The metatrack was divided into 3 stages:

1. Information gathering
   - `monsatan-chatbot`
   - `monsatan-invoices`
   - `monsatan-kiosk`
   - `monsatan-impact-study`
2. Disruption
   - `monsatan-checkmate`
   - `monsatan-defacing`
   - `monsatan-orders`
   - `monsatan-sprinklers`
3. Finale
   - `monsatan-mailserver`

The forum posts for stage 2 were unlocked after solving two of the first stage challenges and the finale was unlocked after solving two of the disruption stage challenges.  

The stages roughly represented the difficulty levels of the challenges: Stage 1 tracks, with the notable exception of `monsatan-impact-study`, were on the easier side; Stage 2 tracks were medium difficulty, and the finale was on the harder side. Impact study was also considered a hard track.  

## What if I just want to experience my annual nosebleed?

1. Disable all AI tools.  
2. Deploy monsatan-mailserver:
```bash
ctf deploy --tracks monsatan-mailserver
```
3. Cry.

## Difference from CTF for monsatan-defacing

1. Self-signed certificates are used instead of ones signed by Northsec
2. When deployed locally, DNS entries likely won't work. It could lead to some links in GitLab not working. You can either set local entries or use the container's IP instead of the `.ctf` domain in the browsers.

## Other challenges and credits

`monsatan-kiosk` was a physical kiosk escape. It cannot really be deployed locally.  
`monsatan-impact-study` was made by [MOBergeron](https://github.com/MOBergeron), who decided to keep the track closed source to avoid spoiling the challenge for other online players.  
`monsatan-defacing` was made by [junior-n30](https://github.com/junior-n30). Included in this repo.
`monsatan-sprinklers` was made by [olipro007](https://github.com/olipro007), however in this case, the code is available directly in this repo to avoid fragmenting the codebase.  
The other challenges in this repo were made by me: [zer0x64](https://github.com/zer0x64)

## Tutorial: set up your local environment

This tutorial was written and tested on Windows using WSL (Ubuntu 24.04). If using Linux directly, you can ignore the WSL-specific steps and adapt if using a non-ubuntu distro.

### 1. Install dependencies

```bash
sudo apt update
sudo apt install python3-pip pipx
pipx ensurepath
```

Close and re-open your terminal.

#### 1.1 Configure WSL

Make sure WSL defaults to version 2:

```bash
wsl --set-default-version 2
```

Check if `systemd` is enabled for WSL2:

```bash
systemctl  # press Q to exit
```

If it says systemd is not enabled, enable systemd for WSL2:

```bash
echo "[boot]
systemd=true" | sudo tee -a /etc/wsl.conf
```

Reboot wsl.

```bash
# IMPORTANT: Make sure to close any VS Code window that is currently open BEFORE running this command.
wsl.exe --shutdown
```

Re-open a WSL shell.

#### 1.2 Ansible

```bash
pipx install --include-deps ansible
pipx inject ansible passlib
ansible --help  # This command MUST work
```

#### 1.3 Incus

Install incus

```bash
sudo apt install --no-install-recommends --yes zfsutils-linux
curl https://pkgs.zabbly.com/get/incus-stable | sudo sh
sudo adduser $USER incus-admin
incus --help  # This command MUST work
```

Reboot WSL to make sure the incus server properly installs.

```bash
wsl.exe --shutdown
```

Re-open a WSL shell.

Initialize Incus

```bash
incus admin init --minimal  # This command has no output.
incus version  # You MUST see the Server version in this output
```

##### Incus and Docker

If you have Docker and Incus installed, there might be networking issues. This is [documented here](https://linuxcontainers.org/incus/docs/main/howto/network_bridge_firewalld/#prevent-connectivity-issues-with-incus-and-docker)

You need to enable IPv4 forwarding.

```bash
echo "net.ipv4.conf.all.forwarding=1" > /etc/sysctl.d/99-forwarding.conf
systemctl restart systemd-sysctl
```
##### Non-debian OS and User Namespace

Read this section only if you're on non-debian OS. Incus uses user namespaces to run unprivileged containers. To do so, it checks for subuid and subgid of the root ([Incus Idmaps for user namespace](https://linuxcontainers.org/incus/docs/main/userns-idmap/)).

However, these may not be defined depending on the OS. The fix is simple, but it may vary from OS to OS. Please read [Incus installation document](https://linuxcontainers.org/incus/docs/main/installing/) and your OS documentation on Incus.

Here's an example for Archlinux: `usermod -v 1000000-1000999999 -w 1000000-1000999999 root`, [source](https://wiki.archlinux.org/title/Incus#Unprivileged_containers)

#### 1.4 OpenTofu (open-source Terraform fork)

```bash
curl -sL https://get.opentofu.org/install-opentofu.sh -o install-opentofu.sh
chmod +x install-opentofu.sh
./install-opentofu.sh --install-method deb
rm -f install-opentofu.sh
tofu --help  # This command MUST work
```

#### 1.5 Git LFS

Install [Git LFS](https://github.com/git-lfs/git-lfs#installing). With a Debian-based or RHEL-based distro, you can install it with the [package repository](https://github.com/git-lfs/git-lfs/blob/main/INSTALLING.md#2-installing-packages) with:

```bash
# Debian/Ubuntu
sudo apt-get install git-lfs

# RHEL
sudo yum install git-lfs
```

### 2. Deploy a track locally

```bash
ctf deploy --tracks monsatan-chatbot
```

If you have an error here regarding storage pool, create a default incus storage pool:

```bash
incus storage create default dir
```

To deploy all the tracks contained in this repository, simply run:

```bash
ctf deploy -t monsatan-chatbot -t monsatan-checkmate -t monsatan-invoices -t monsatan-mailserver -t monsatan-orders -t monsatan-sprinklers
```

### 3. Remove tracks

To clean up the tracks, simply run:

```bash
ctf destroy
```
