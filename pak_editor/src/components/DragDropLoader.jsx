import { onMount } from 'solid-js';

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
          const arrayBuffer = e.target.result;

          // Call WebAssembly parser module
          const response = await fetch('/pak_parser');
          const parserModule = await WebAssembly.compile(response);
          const instance = await WebAssembly.instantiate(parserModule, {
            memory: new WebAssembly.Memory({ initial: 256 })
          });

          // Process file using WebAssembly
          // Implement parsing logic integration
          console.log('Parsing .pak file...');
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