import { onMount } from 'solid-js';
import init, { parse_pak_js } from '../../pkg/pak_parser.js';

export default function DragDropLoader() {
  onMount(() => {
    const dropZone = document.getElementById('drop-zone');

    ['dragenter', 'dragover'].forEach(eventName => {
      dropZone.addEventListener(eventName, (e) => {
        e.preventDefault();
        e.stopPropagation();
        dropZone.classList.add('hover');
      }, false);
    });

    ['dragleave', 'drop'].forEach(eventName => {
      dropZone.addEventListener(eventName, (e) => {
        e.preventDefault();
        e.stopPropagation();
        dropZone.classList.remove('hover');
      }, false);
    });

    dropZone.addEventListener('drop', (e) => {
      const file = e.dataTransfer.files[0];
      if (!file) return;

      const reader = new FileReader();
      reader.onload = async (e) => {
        try {
          await init();
          const bytes = new Uint8Array(e.target.result);
          const count = parse_pak_js(bytes);
          console.log(`Parsed ${count} entries`);
        } catch (error) {
          console.error('Error loading .pak file:', error);
        }
      };

      reader.readAsArrayBuffer(file);
    });
  });

  return (
    <div id="drop-zone" class="drop-zone">
      <h3>Drop .pak file here</h3>
      <p>or click to select file</p>
      <input type="file" accept=".pak" />
    </div>
  );
}
