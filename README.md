# BeamMP Server Manager

BeamMP Server Manager gives a UI to the BeamMP server. It is intended for use in small-scale, self-hosted servers. In the manager, you can add and remove mods (vehicles and maps), as well as select the current map. It has no relation to and is not endorsed in any way by the creators of the BeamMP mod or the BeamMP Server.

## Installation

### General

For the server to be connectable to people outside of your local network, it must be port-forwarded. A guide on how to do this can be found [here](https://www.noip.com/support/knowledgebase/general-port-forwarding-guide). The default port that the BeamMP Server uses is `30184`, however, this can be changed in the server settings if you wish. Please be aware that port-forwarding on your home network can be dangerous as it could allow someone to gain unauthorized access to your network.

If you already have a BeamMP server set up and wish to integrate the server manager with it, follow the instructions [here](#integrating-with-existing-server).

### Windows

1. Download the `BeamMP-Server-Manager.exe` file from the releases tab.
2. Place the file in a folder on your computer.
3. Generate an AuthKey. A guide can be found [here](https://docs.beammp.com/server/create-a-server/#2-obtaining-an-authentication-key).
4. Open the manager file. (A terminal window may open and then close after a few seconds. This is normal.)
5. Copy your generated AuthKey.
6. Paste your AuthKey where prompted.
7. You can now click the `Start` button and the server will start normally.

### Linux

**There is currently no Linux executable available for download!**

The only supported Linux distributions are Ubuntu and Debian. Both require `liblua5.3` to be installed to work correctly.

Linux installation is identical to Windows except for the file downloaded from the releases, so follow the instructions for Windows but be sure to download the correct file from the releases tab.

### Integrating With Existing Server

1. Download the manager executable for your OS.
2. Place the manager executable in the folder where your server executable is.
3. Run the manager. Everything will integrate automatically.

## Usage

### Server Control

Whenever the server manager is opened, the server is also started. The controls for the server are in the top bar to the right. The `Restart` button will flash whenever something has been changed that requires a server restart to take effect. You can also manually start and stop the server if you wish. The server closes automatically when the server manager is closed.

### Map Selection

To select a map, just double-click on it. For the change to take effect, the server must be restarted using the `Restart` button.

### Modded Content

#### Installation

Any modded content (map, vehicle, or otherwise) can be installed by dragging and dropping the `.zip` file into the manager. It should automatically be added to either the map list or vehicle list depending on the type of content it is. If it is not, it is most likely improperly formatted or otherwise broken.

#### Activation

Modded content can be in either an activated or deactivated state. If it is activated, it is loaded onto the server and able to be used. If it is deactivated, it is not loaded onto the server and is unable to be used, but it is not deleted. Activation status can be changed using the slider at the top right of each mod. Modded maps are automatically deactivated to shorten loading times for people connecting to the server unless the modded map is currently in use.

#### Deletion

Modded content can be deleted at any time by clicking on the trashcan icon on the bottom right of a mod.

#### Multi-Vehicle Mods

Some mods contain multiple vehicles or variants of vehicles within a single `.zip`. If this is the case, the full list of content in the mod can be viewed by clicking on the arrow on the left of the mod. If there is no arrow, it is not a multi-vehicle mod.

## Settings

The settings page can be accessed by clicking the settings cog at the bottom right.

### Server Settings

The most relevant server settings can be found in the settings page. Changes are saved automatically, but the server must be restarted for the changes to take effect. To view and change the full list of settings for the server, click the `Open Server Config File` button at the bottom of the page.

### Manager Settings

- Toggle Automatic Server Updates (default: on)
- Show Server Terminal (default: off)

> The "Show Server Terminal" setting only has an effect on Windows. On Linux, the terminal will always be shown.

Like the server settings, changes are saved automatically.

## Joining The Server

Click [here](https://docs.beammp.com/server/create-a-server/#6-how-to-join-your-server) for information on how you and others can join your server.

## Updating The Server

If automatic updating is enabled in the settings, the BeamMP Server will update as soon as a new version is detected. A new version is checked for each time the manager is started.

## Building from Source

### Prerequisites

- Rust
- NPM
- Tauri CLI (installable with the following command: `npm install --save-dev @tauri-apps/cli@">1.0.0"`)

### Building

1. Clone this repository onto your computer (`git clone https://github.com/Finn-Kehoe/BeamMP-Server-Manager.git`).
2. Navigate to the repository folder in a terminal application.
3. Execute the command `npm run tauri build`.
4. The executable can be found (from project root) at `src-tauri/target/release/BeamMP-Server-Manager.exe`. It's portable, so you can take it from here and place it wherever you like before using. To set up and use the manager, refer to the [Installation](#installation) section.

## Additional Information

For additional information about BeamMP and the BeamMP Server itself, check out the [BeamMP Website](https://www.beammp.com/) and the [BeamMP Server Github Repo](https://github.com/beammp/beammp-server).
