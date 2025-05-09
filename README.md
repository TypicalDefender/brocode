# Brocode

A modular Rust CLI tool that enhances developer workflows, starting with AI-powered commit message generation.

## Features

- AI-powered commit message generation based on your code changes
- User-friendly prompts and confirmations
- Configurable OpenAI settings
- Modular architecture for future extensions

## Installation

### Prerequisites

- Git installed
- OpenAI API key (get one from [OpenAI's platform](https://platform.openai.com/api-keys))

### Option 1: Install Rust and build from source

1. Install Rust and Cargo (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"  # Add Cargo to your PATH for the current session
```

2. Clone the repository:
```bash
git clone https://github.com/yourusername/brocode.git
cd brocode
```

3. Build and install globally:
```bash
cargo install --path .
```

This will install the `brocode` command to `~/.cargo/bin/`, which should be in your PATH.

### Option 2: Install pre-built binary (Coming soon)

## Configuration

Brocode requires an OpenAI API key to generate commit messages. You can configure it in two ways:

1. **Environment variable** (recommended for temporary use):
```bash
export OPENAI_API_KEY="your-api-key-here"
```

2. **Configuration file** (recommended for permanent use):
```bash
mkdir -p ~/.config/brocode
```

Create or edit `~/.config/brocode/config.toml` with the following content:
```toml
[openai]
api_key = "your-api-key-here"
model = "gpt-3.5-turbo"
temperature = 0.7
max_tokens = 300
system_prompt = "You are a helpful assistant who writes concise, professional Git commit messages. Given code changes, write a commit message with a clear single-line summary followed by a more detailed description in Markdown that explains what was changed and why."
```

The first time you run Brocode, it will create a default configuration file if one doesn't exist.

## Usage

### Generating AI-powered commit messages

1. Make changes to your code in a git repository
2. Stage your changes with `git add`
3. Run the commit command:
```bash
brocode commit
```

Brocode will:
- Analyze your staged changes
- Generate a commit message using OpenAI
- Show you the generated message
- Allow you to edit it if needed
- Commit the changes with the approved message

### Using a manual commit message

If you prefer to provide your own commit message:
```bash
brocode commit -m "Your commit message here"
```

## Development

Brocode is built with a modular architecture to make it easy to add new features. The current implementation focuses on commit message generation, but the framework is designed to support additional developer workflow enhancements in the future.

## Troubleshooting

### Command not found: brocode

If you installed with `cargo install --path .` but the `brocode` command is not found, ensure that `~/.cargo/bin` is in your PATH:

For bash/zsh:
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
source ~/.bashrc  # or ~/.zshrc
```

### OpenAI API key issues

If you see an error about the OpenAI API key:

1. Make sure you've set up your API key either in the config file or as an environment variable
2. Verify that your API key is valid and has not expired
3. Check that you have sufficient credits in your OpenAI account

### Git repository issues

Brocode needs to be run from within a git repository. If you see git-related errors:

1. Make sure you're in a git repository
2. Ensure you have changes that need to be committed
3. Check that git is properly installed and configured

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
