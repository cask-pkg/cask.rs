#!/bin/sh

set -e

owner="cask-pkg"
repo="cask.rs"
exe_name="cask"
githubUrl="https://github.com"
version="" # auto install the latest version

get_arch() {
    # darwin/amd64: Darwin axetroydeMacBook-Air.local 20.5.0 Darwin Kernel Version 20.5.0: Sat May  8 05:10:33 PDT 2021; root:xnu-7195.121.3~9/RELEASE_X86_64 x86_64
    # linux/amd64: Linux test-ubuntu1804 5.4.0-42-generic #46~18.04.1-Ubuntu SMP Fri Jul 10 07:21:24 UTC 2020 x86_64 x86_64 x86_64 GNU/Linux
    a=$(uname -m)
    case ${a} in
        "x86_64" | "amd64" )
            echo "x86_64"
        ;;
        "aarch64" | "arm64")
            echo "aarch64"
        ;;
        "mips64el")
            echo "mips64el"
        ;;
        "mips64")
            echo "mips64"
        ;;
        *)
            echo ${NIL}
        ;;
    esac
}

get_os(){
    # darwin: Darwin
    # linux: Linux
    os=$(uname -s | awk '{print tolower($0)}')
    echo $os
}

get_vendor(){
    # darwin: Darwin
    os=$(get_os)
    case ${os} in
        "darwin" )
            echo "apple"
        ;;
        *)
            echo "unknown"
        ;;
    esac
}

get_abi(){
    vendor=$(get_vendor)
    case ${vendor} in
        "apple" )
            echo ""
        ;;
        *)
            ldd=$(cat '/usr/bin/ldd')
            if [ "$ldd" == *"musl"* ]; then
                echo "-musl"
            else
                echo "-gnu"
            fi
        ;;
    esac
}

downloadFolder="${HOME}/Downloads"
mkdir -p ${downloadFolder} # make sure download folder exists
os=$(get_os)
arch=$(get_arch)
vendor=$(get_vendor)
abi=$(get_abi)
file_name="${exe_name}-${arch}-${vendor}-${os}${abi}.tar.gz" # the file name should be download
downloaded_file="${downloadFolder}/${file_name}" # the file path should be download
executable_folder="/usr/local/bin" # Eventually, the executable file will be placed here

# if version is empty
if [ -z "$version" ]; then
    asset_path=$(
        command curl -sSf ${githubUrl}/${owner}/${repo}/releases |
        command grep -o "/${owner}/${repo}/releases/download/.*/${file_name}" |
        command head -n 1
    )
    if [[ ! "$asset_path" ]]; then exit 1; fi
    asset_uri="${githubUrl}${asset_path}"
else
    asset_uri="${githubUrl}/${owner}/${repo}/releases/download/${version}/${file_name}"
fi

echo "[1/3] Download ${asset_uri} to ${downloadFolder}"
rm -f ${downloaded_file}
curl --fail --location --output "${downloaded_file}" "${asset_uri}"

echo "[2/3] Install ${exe_name} to the ${executable_folder}"
tar -xz -f ${downloaded_file} -C ${executable_folder}
exe=${executable_folder}/${exe_name}
chmod +x ${exe}

echo "[3/3] Set environment variables"
echo "${exe_name} was installed successfully to ${exe}"
if command -v $exe_name --version >/dev/null; then
    echo "Run '$exe_name --help' to get started"
else
    echo "Manually add the directory to your \$HOME/.bash_profile (or similar)"
    echo "  export PATH=${executable_folder}:\$PATH"
    echo "Run '$exe_name --help' to get started"
fi

exit 0