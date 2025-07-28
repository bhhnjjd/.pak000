import { createSignal } from 'solid-js';

export default function HexEditor({ data }) {
  const [offset, setOffset] = createSignal(0);
  const [selection, setSelection] = createSignal({ start: 0, end: 0 });

  const hexView = () => {
    const view = [];
    for (let i = offset(); i < Math.min(offset() + 16, data.length); i += 1) {
      const hex = data[i].toString(16).padStart(2, '0');
      const ascii = String.fromCharCode(data[i]);
      view.push(`\n${hex} (${ascii})`);
    }
    return view.join('');
  };

  return (
    <div class="hex-editor">
      <textarea
        value={hexView()}
        onSelectionChange={(e) => {
          const start = e.target.selectionStart;
          const end = e.target.selectionEnd;
          setSelection({ start, end });
        }}
      />
      <div class="ascii-preview">
        {Array.from(data.slice(offset(), offset() + 16))
          .map(c => String.fromCharCode(c))
          .join('')}
      </div>
      <div class="navigation">
        <button onClick={() => setOffset(offset() - 16)}>Previous</button>
        <button onClick={() => setOffset(offset() + 16)}>Next</button>
      </div>
    </div>
  );
}
