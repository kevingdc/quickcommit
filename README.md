# quickcommit

quickcommit is a command-line tool that auto-generates commit messages for you.

## Installation

To install quickcommit, follow these steps:

1. Download the latest release of quickcommit from the [Releases](https://github.com/kevingdc/quickcommit/releases) page.
2. Extract the downloaded archive to a directory of your choice.
3. Open a terminal and navigate to the directory where you extracted quickcommit.
4. Run the following command to make quickcommit executable:

```bash
chmod +x quickcommit
```

5. Add the quickcommit executable to your system's PATH variable. This will allow you to run quickcommit from any directory.

- For Windows:
  - Move the `quickcommit` executable to the `C:\Program Files` directory.
  - (Optional) You can rename the command to `qc` so it's shorter to type.
  - Open the Start menu and search for "Environment Variables".
  - Click on "Edit the system environment variables".
  - In the System Properties window, click on the "Environment Variables" button.
  - In the "System variables" section, select the "Path" variable and click on "Edit".
  - Click on "New" and enter `C:\Program Files` as the new path.
  - Click "OK" to save the changes.
  - You can now run quickcommit from any directory in the command prompt.

- For macOS and Linux:
  - Move the `quickcommit` executable to the `/usr/local/bin` directory.
  - Open a terminal.
  - Run the following command to open the `.bash_profile` file:
    ```bash
    nano ~/.bash_profile
    ```
  - Add the following line at the end of the file:
    ```bash
    export PATH="/usr/local/bin:$PATH"
    ```
  - (Optional) You can also add an alias, like `qc` so that it's shorter:
    ```bash
    alias qc="quickcommit
    ```
  - Press `Ctrl + X` to exit nano, then press `Y` to save the changes.
  - Run the following command to apply the changes:
    ```bash
    source ~/.bash_profile
    ```
  - You can now run quickcommit from any directory in the terminal.

6. Get an [OpenAI API Key](https://platform.openai.com/api-keys).
7. Set it using the `quickcommit set-api-key` command.

## Usage

Once quickcommit is installed, you can use it by running the `quickcommit` command in your terminal. This will commit all staged files with an auto-generated message.

To check other commands, you can run:
```
quickcommit help
```

That's it! You're now ready to use quickcommit to streamline your Git workflow. If you have any questions or comments, you can message me on Twitter [@kevingdchan](https://twitter.com/kevingdchan).
