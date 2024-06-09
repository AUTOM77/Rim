# Rim

[![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/AUTOM77/Rim/ci.yml)](https://github.com/AUTOM77/Rim/actions)
[![GitHub license](https://img.shields.io/github/license/AUTOM77/Rim)](./LICENSE)
[![GitHub contributors](https://img.shields.io/github/contributors/AUTOM77/Rim)](https://github.com/AUTOM77/Rim/graphs/contributors)
[![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/m/AUTOM77/Rim)](https://github.com/AUTOM77/Rim/commits)
[![GitHub top language](https://img.shields.io/github/languages/top/AUTOM77/Rim?logo=rust&label=)](./rim-cli/Cargo.toml#L4)
[![Open Issues](https://img.shields.io/github/issues/AUTOM77/Rim)](https://github.com/AUTOM77/Rim/issues)
[![Code Size](https://img.shields.io/github/languages/code-size/AUTOM77/Rim)](.)
[![GitHub all releases](https://img.shields.io/github/downloads/AUTOM77/Rim/total?logo=github)](https://github.com/AUTOM77/Rim/releases)  
[![GitHub release (with filter)](https://img.shields.io/github/v/release/AUTOM77/Rim?logo=github)](https://github.com/AUTOM77/Rim/releases)

> Rim, a Rust based Multi-Modal Hyper Caption Tool in Parallel, v3.0 released!

### Usage

> [!TIP]
> `Rim` now combine Single and Batch Caption into one commandline <br/>
> Use `rim ${path} -c config.toml --limit 10 --qps 10` instead.

For a single key on single project, we recommend using `rim ${path} -c config.toml --limit 360`.

<details>
  <summary>Old Usage</summary>

1. **Single Image/Video Captioning:**

```bash
rim -f ${file_path} -c `config.toml`
```
Rim generates a `*.txt` file containing the caption for a single image or video.

2. **Batch Image/Video Captioning:**

```bash
rim -d ${dir_path} -c `config.toml`
```

For a directory of images or videos, `Rim` generates a corresponding list of `*.txt` caption files.
</details>

---

1. Rim will now generates a folder called `xxx_cap` contains `*.txt` caption files.
2. Sample `config.toml` can be found in [config.toml](./config.toml)

### Config

Creating a Sample Configuration (Unix):

```dash
cat <<EOF | tee config.toml
[prompt]
value = "Caption this video"

[azure]
api = [
    ['https://closedAI-1.openai.azure.com', 'sk-00000000000000000000000000000000', 'gpt-4o'],
    ['https://closedAI-2.openai.azure.com', 'sk-00000000000000000000000000000001', 'gpt-4v']
]

[gemini]
api = [
    ['https://generativelanguage.googleapis.com', 'AIza00000000000000000000000000000000000', 'gemini-1.5-flash-latest'],
    ['https://generativelanguage.googleapis.com', 'AIza00000000000000000000000000000000001', 'gemini-1.5-pro-latest'],
]
EOF
```

### Nightly Build

```sh
curl -fsSL https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
rustup update nightly && rustup default nightly

cargo build --release
./target/release/rim "assets/images" -c config.toml
```

### Nightly Build with mirror
```sh
curl -fsSL https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
echo """
[source.crates-io]
replace-with = 'mirror'

[source.mirror]
registry = 'sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/'
""" | tee ${CARGO_HOME:-$HOME/.cargo}/config.toml
rustup update nightly && rustup default nightly

cargo build --release
./target/release/rim "assets/images" -c config.toml
```


# Upload a file using the GenAI File API via curl.

```bash
api_key=""
input_file=""
display_name=""

while getopts a:i:d: flag
do
case "${flag}" in
a) api_key=${OPTARG};;
i) input_file=${OPTARG};;
d) display_name=${OPTARG};;
esac
done

BASE_URL="https://generativelanguage.googleapis.com"

CHUNK_SIZE=8388608 # 8 MiB
MIME_TYPE=$(file -b --mime-type "${input_file}")
NUM_BYTES=$(wc -c < "${input_file}")

echo "Starting upload of '${input_file}' to ${BASE_URL}..."
echo "  MIME type: '${MIME_TYPE}'"
echo "  Size: ${NUM_BYTES} bytes"

# Initial resumable request defining metadata.

tmp_header_file=$(mktemp /tmp/upload-header.XXX)
curl "${BASE_URL}/upload/v1beta/files?key=${api_key}" \
-D "${tmp_header_file}" \
-H "X-Goog-Upload-Protocol: resumable" \
-H "X-Goog-Upload-Command: start" \
-H "X-Goog-Upload-Header-Content-Length: ${NUM_BYTES}" \
-H "X-Goog-Upload-Header-Content-Type: ${MIME_TYPE}" \
-H "Content-Type: application/json" \
-d "{'file': {'display_name': '${display_name}'}}"
upload_url=$(grep "x-goog-upload-url: " "${tmp_header_file}" | cut -d" " -f2 | tr -d "\r")
rm "${tmp_header_file}"

if [[ -z "${upload_url}" ]]; then
echo "Failed initial resumable upload request."
exit 1
fi

# Upload the actual bytes.

NUM_CHUNKS=$(((NUM_BYTES + CHUNK_SIZE - 1) / CHUNK_SIZE))
tmp_chunk_file=$(mktemp /tmp/upload-chunk.XXX)
for i in $(seq 1 ${NUM_CHUNKS})
do
offset=$((i - 1))
byte_offset=$((offset * CHUNK_SIZE))

# Read the actual bytes to the tmp file.

dd skip="${offset}" bs="${CHUNK_SIZE}" count=1 if="${input_file}" of="${tmp_chunk_file}" 2>/dev/null
num_chunk_bytes=$(wc -c < "${tmp_chunk_file}")
upload_command="upload"
if [[ ${i} -eq ${NUM_CHUNKS} ]] ; then

# For the final chunk, specify "finalize".

upload_command="${upload_command}, finalize"
fi
echo "  Uploading ${byte_offset} - $((byte_offset + num_chunk_bytes)) of ${NUM_BYTES}..."
curl "${upload_url}" \
-H "Content-Length: ${num_chunk_bytes}" \
-H "X-Goog-Upload-Offset: ${byte_offset}" \
-H "X-Goog-Upload-Command: ${upload_command}" \
--data-binary "@${tmp_chunk_file}"
done

rm "${tmp_chunk_file}"

echo "Upload complete!"
```