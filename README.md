# path-ai - opening a folder using AI
This command allows you to open a folder found by description using AI.

## Help
```shell
# [--key, -k] - set openai key
path-ai --key 1234567890qwertyuiopasdfghjklzxcvbnm

# [--model, -m] - set gpt model, default: gpt-3.5-turbo-1106
path-ai --model gpt-4-1106-preview

# [--scan, -s] - scan folders from the selected indexes.txt file
path-ai --scan

# [--version, -v] - view path-ai version
path-ai --version

# [--help, -h] - view help
path-ai --help

# path-ai [description of the folder and program in which you want to open it]
path-ai open terminal on desktop
```

Create indexes.txt file in the same folder as path-ai.exe
indexes.txt format:
```
C:/::1*
D:/::2*
C:/Users/user/Desktop/::1*
```

## Install
```shell
cargo build --release
mkdir build

# liunx
echo /::1* > build/indexes.txt
cp target/release/path-ai build/path-ai
export PATH=$PATH:$(pwd)/build

# windows
echo C:/::1* > build/indexes.txt
copy target/release/path-ai.exe build/path-ai.exe
set PATH=%PATH%;%cd%/build
```

## Add this tool to ueli
- Install the vexec tool to run the command with text to voice conversion: https://github.com/binqbit/vexec
- Open ueli settings
- Go to the tab "Shortcuts"
- Add new shortcut with name "Path AI" and command "vexec --ueli path-ai"
