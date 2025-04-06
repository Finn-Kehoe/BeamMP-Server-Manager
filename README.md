# Overview
BeamMP Server Manager gives a UI to the BeamMP server. It is intended for use in small-scale, self-hosted servers. In the manager, you can add and remove mods (vehicles and maps), as well as select the current map. It has no relation to and is not endorsed in any way by the creators of the BeamMP mod or the BeamMP Server.

# Installation
## General
For the server to be connectable to people outside of your local network, it must be port-forwarded. A guide on how to do this can be found [here](https://www.noip.com/support/knowledgebase/general-port-forwarding-guide). The default port that the BeamMP Server uses is `30184`, however, this can be changed in the server settings if you wish. Please be aware that port-forwarding on your home network can be dangerous as it could allow someone to gain unauthorized access to your network.
## Windows
1. Download the `BeamMP-Server-Manager.exe` file from the releases tab.
2. Place the file wherever you want on your computer (highly recommended to put it in a folder).
3. Run the executable. It should immediately close, and a file called `ServerConfig.toml` will appear.
4. Generate an AuthKey. A guide can be found [here](https://docs.beammp.com/server/create-a-server/#2-obtaining-an-authentication-key).
5. Copy your generated AuthKey and paste it into the quotation marks in the `AuthKey` field of `ServerConfig.toml`.
6. You can now open the server manager executable, and it will run properly.
## Linux
**There is currently no Linux executable available for download!**

The only supported Linux distributions are Ubuntu and Debian. Both require `liblua5.3` to be installed to work correctly.

Linux installation is identical to Windows except for the file downloaded from the releases, so follow the instructions for Windows but be sure to download the correct file from the releases tab.

# Usage
## Server Control
Whenever the server manager is opened, the server is also started. The controls for the server are in the top bar to the right. The `Restart` button will flash whenever something has been changed that requires a server restart to take effect. You can also manually start and stop the server if you wish. The server closes automatically when the server manager is closed.

## Map Selection
To select a map, just double-click on it. For the change to take effect, the server must be restarted using the `Restart` button.

## Modded Content
### Installation
Any modded content (map, vehicle, or otherwise) can be installed by dragging and dropping the `.zip` file into the manager. It should automatically be added to either the map list or vehicle list depending on the type of content it is. If it is not, it is most likely improperly formatted or otherwise broken.

### Activation
Modded content can be in either an activated or deactivated state. If it is activated, it is loaded onto the server and able to be used. If it is deactivated, it is not loaded onto the server and is unable to be used, but it is not deleted. Activation status can be changed using the slider at the top right of each mod. Modded maps are automatically deactivated to shorten loading times for people connecting to the server unless the modded map is currently being played on.

### Deletion
Modded content can be deleted at any time by clicking on the trashcan icon on the bottom right of a mod.

### Multi-Vehicle Mods
Some mods contain multiple vehicles or variants of vehicles within a single `.zip`. If this is the case, the full list of content in the mod can be viewed by clicking on the arrow on the left of the mod. If there is no arrow, it is not a multi-vehicle mod.

## Server Settings
The settings for the BeamMP Server can be accessed by clicking on the settings cog at the bottom right. It will open the settings file in your default text editor. Please keep in mind that any change of these settings will require a restart of the server.

# Joining The Server
Click [here](https://docs.beammp.com/server/create-a-server/#6-how-to-join-your-server) for information on how you and others can join your server.

# Updating The Server
The BeamMP Server will automatically update as soon as a new version is detected. A new version is checked for each time the manager is started.

# Building from Source
## Prerequisites
- Rust
- NPM
- Tauri CLI (installable with the following command: `npm install --save-dev @tauri-apps/cli@">1.0.0"`)
## Building
1. Clone this repository onto your computer (`git clone https://github.com/Finn-Kehoe/BeamMP-Server-Manager.git`).
2. Navigate to the repository folder in a terminal application.
3. Execute the command `npm run tauri build`.
4. The executable can be found (from project root) at `src-tauri/target/release/BeamMP-Server-Manager.exe`. It's portable, so you can take it from here and place it wherever you like before using. To set up and use the manager, refer to the [Installation](#installation) section.

# Additional Information
For additional information about BeamMP and the BeamMP Server itself, check out the [BeamMP Website](https://www.beammp.com/) and the [BeamMP Server Github Repo](https://github.com/beammp/beammp-server).