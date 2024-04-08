import hljs from 'https://unpkg.com/@highlightjs/cdn-assets@11.9.0/es/highlight.min.js';
import bash from 'https://unpkg.com/@highlightjs/cdn-assets@11.9.0/es/languages/bash.min.js';
import css from 'https://unpkg.com/@highlightjs/cdn-assets@11.9.0/es/languages/css.min.js';
import plaintext from 'https://unpkg.com/@highlightjs/cdn-assets@11.9.0/es/languages/plaintext.min.js';
import rust from 'https://unpkg.com/@highlightjs/cdn-assets@11.9.0/es/languages/rust.min.js';

hljs.registerLanguage('bash', bash);
hljs.registerLanguage('css', css);
hljs.registerLanguage('plaintext', plaintext);
hljs.registerLanguage('rust', rust);

export function highlightElement(element) {
    hljs.highlightElement(element);
}
