# 📊 GitLab Activity Plugin for MOT

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) <!-- Assuming MIT, replace if different -->

Integrates your GitLab commit activity directly into [MOT](https://github.com/Tuurlijk/mot), allowing you to view and potentially import commit data as time entries.

## ✨ Features

*   🦊 Fetches commit data from one or more GitLab instances (self-hosted or gitlab.com).
*   👤 Filters commits based on author email.
*   📅 Displays commit history within MOT's weekly view.
*   🔌 Seamlessly integrates with the MOT plugin system.
*   ⚙️ Configurable via a simple TOML file.
*   🐧 Includes an easy-to-use installation script for Linux/macOS.
*   ⚡️ **In-Memory Caching**: Improves performance for repeated requests of past date ranges by caching results in memory. The cache is cleared when the plugin process exits.

## 🚀 Installation

This plugin is designed to be installed using the provided script within its distribution package.

### Prerequisites

*   **MOT**: You must have MOT installed and configured. See the [MOT Installation Guide](https://github.com/Tuurlijk/mot#installation).
*   **GitLab Personal Access Token**: You need a Personal Access Token from each GitLab instance you want to connect to. The token requires the `read_api` scope.
    *   Create one at: `https://[your_gitlab_host]/-/profile/personal_access_tokens` (replace `[your_gitlab_host]` with your GitLab instance's hostname, e.g., `gitlab.com`).

### Steps (Linux / macOS)

1.  **Download and Extract**: Obtain the plugin release package (e.g., `mot-plugin-gitlab-vX.Y.Z.tar.gz` or `.zip`) and extract it. You should find these core files:
    *   `mot-plugin-gitlab` (the executable)
    *   `manifest.toml` (plugin metadata)
    *   `config.toml` (configuration template)
    *   `install.sh` (installation script)
    *   `Readme.md` (this file)

2.  **Navigate**: Open your terminal and `cd` into the extracted directory:
    ```bash
    cd /path/to/extracted/mot-plugin-gitlab
    ```

3.  **Run Installer**: Execute the installation script:
    ```bash
    bash ./install.sh
    ```
    *   This copies the plugin files (`mot-plugin-gitlab`, `manifest.toml`, `config.toml`) to the correct Mot plugin directory (`~/.config/mot/plugins/mot-plugin-gitlab/`).
    *   It sets execute permissions for the plugin binary.
    *   **Note**: If a `config.toml` already exists in the destination, it will *not* be overwritten.

4.  **Configure**: Proceed to the [Configuration](#🔧-configuration) section below. This step is essential.

5.  **Restart MOT**: Close and reopen MOT to load the new plugin.

## 🔧 Configuration

After installation, you **must** configure the plugin by editing its configuration file.

*   **Location**: The configuration file is located at:
    *   Linux/macOS: `~/.config/mot/plugins/mot-plugin-gitlab/config.toml`
    *   Windows: `%APPDATA%\mot\plugins\mot-plugin-gitlab\config.toml` (Manual installation might be needed for Windows currently)

*   **Editing**: Open the `config.toml` file in a text editor. You need to provide details for each GitLab instance you wish to monitor within `[[clients]]` blocks.

    ```toml
    # Example config.toml for mot-plugin-gitlab

    # Global setting for the plugin (part of manifest/standard plugin config)
    enabled = true

    # Define one or more GitLab instances to connect to
    [[clients]]
    id = "gitlab-com" # Unique identifier for this client block
    name = "GitLab.com Personal" # User-friendly name shown in MOT (optional)
    gitlab_host = "gitlab.com"
    gitlab_token = "YOUR_GITLAB.COM_ACCESS_TOKEN" # Replace with your actual token
    author_email = "your-email@example.com" # Replace with the email used in your commits

    [[clients]]
    id = "gitlab-work"
    name = "Work Self-Hosted GitLab"
    gitlab_host = "gitlab.company.com" # Replace with your company's GitLab host
    gitlab_token = "YOUR_WORK_ACCESS_TOKEN"
    author_email = "your-work-email@company.com"

    # Add more [[clients]] blocks if needed
    ```

*   **Details**:
    *   `enabled`: Set to `true` to enable the plugin within MOT (requires restart).
    *   `id`: A unique string to identify this configuration block (e.g., "gitlab-com", "work-gitlab").
    *   `name`: A user-friendly name displayed in MOT (optional).
    *   `gitlab_host`: The hostname of your GitLab instance (e.g., `gitlab.com` or `gitlab.yourcompany.com`).
    *   `gitlab_token`: Your GitLab Personal Access Token with `read_api` scope.
    *   `author_email`: The email address associated with your Git commits on this instance. The plugin will only fetch commits matching this email.

*   **Save** the changes to the file.

## 📖 Usage

Once installed, configured, and enabled:

1.  **Restart MOT**.
2.  Navigate to the **Plugins** view (press `p` in the main view).
3.  Ensure the "GitLab Activity Plugin" is listed and **enabled** (use `Space` to toggle if needed, then restart MOT).
4.  Return to the main time entry view (`Esc` or `p`).
5.  Your GitLab commit activity (matching the configured email(s)) should appear alongside your regular MoneyBird time entries, tagged with the source plugin.
6.  (Optional) If MOT supports importing plugin entries (check MOT features/roadmap), you might be able to select GitLab entries and import them (`i` key in standard MOT).

Refer to the main MOT documentation for general navigation and interaction within the application.

## 🗑️ Uninstallation

To remove the plugin:

1.  Delete the plugin's directory:
    ```bash
    # Linux / macOS
    rm -rf ~/.config/mot/plugins/mot-plugin-gitlab

    # Windows (adjust path as needed)
    # rmdir /s /q %APPDATA%\mot\plugins\mot-plugin-gitlab
    ```
2.  Restart MOT.

## 📜 License

This project is likely licensed under the MIT License - check the LICENSE file in the repository for confirmation.

## 🤝 Contributing

Contributions are welcome! Please refer to the main MOT contribution guidelines if applicable, or open an issue/pull request on this plugin's repository.

## 🙏 Acknowledgements

* The developers of [MOT](https://github.com/Tuurlijk/mot).
* GitLab Inc. for providing the API. 
* The developers of [apisnip](https://github.com/Tuurlijk/apisnip). Used to trim the Gitlab api down to manageable size for progenitor.
* The developers of [progenitor](https://github.com/oxidecomputer/progenitor). Used to generate the API code from the openapi specification.