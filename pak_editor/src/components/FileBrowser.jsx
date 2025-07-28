import { createSignal } from 'solid-js';

export default function FileBrowser() {
  const [files, setFiles] = createSignal([]);
  const [selectedFile, setSelectedFile] = createSignal(null);

  const handleFileLoad = (event) => {
    const file = event.target.files[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = async (e) => {
      try {
        const arrayBuffer = e.target.result;
        const module = await WebAssembly.instantiate(arrayBuffer, {
          env: {
            memory: new WebAssembly.Memory({ initial: 256 }),
          },
        });

        const result = module.instance.exports.parse_pak();
        setFiles(result.entries);
      } catch (error) {
        console.error('Error parsing .pak file:', error);
      }
    };

    reader.readAsArrayBuffer(file);
  };

  return (
    <div class="file-browser">
      <input type="file" accept=".pak" onInput={handleFileLoad} />
      <ul>
        {files().map((file) => (
          <li
            key={file.name}
            class={file === selectedFile() ? 'selected' : ''}
            onClick={() => setSelectedFile(file)}
          >
            {file.name} ({file.size} bytes)
          </li>
        ))}
      </ul>
    </div>
  );
}