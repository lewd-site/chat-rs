import { Schema, DOMParser, Mark } from 'prosemirror-model';
import { EditorState, Plugin } from 'prosemirror-state';
import { EditorView } from 'prosemirror-view';
import { undo, redo, history } from 'prosemirror-history';
import { keymap } from 'prosemirror-keymap';
import { baseKeymap, toggleMark, selectAll, deleteSelection } from 'prosemirror-commands';

class Tooltip {
  private readonly tooltip: HTMLElement;

  public constructor(private readonly view: EditorView) {
    this.tooltip = document.createElement('div');
    this.tooltip.className = 'tooltip';
    this.tooltip.innerHTML = `
        <div class="tooltip__row">
            <button type="button" class="tooltip__bold" data-mark="bold">B</button>
            <button type="button" class="tooltip__italic" data-mark="italic">I</button>
            <button type="button" class="tooltip__underline" data-mark="underline">U</button>
            <button type="button" class="tooltip__strike" data-mark="strike">S</button>
            <button type="button" class="tooltip__sub" data-mark="subscript"><span>Sub</span></button>
            <button type="button" class="tooltip__sup" data-mark="superscript"><span>Sup</span></button>
        </div>
        <div class="tooltip__row">
            <button type="button" class="tooltip__code" data-mark="code">Code</button>
            <button type="button" class="tooltip__codeblock" data-mark="codeblock">Code Block</button>
            <button type="button" class="tooltip__spoiler" data-mark="spoiler">Spoiler</button>
        </div>`;
    this.tooltip.addEventListener('click', this.handleClick);

    view.dom.parentNode!.appendChild(this.tooltip);

    this.update(view, null);
  }

  private handleClick = (e: MouseEvent) => {
    e.preventDefault();

    if (!(e.target instanceof HTMLElement)) {
      return;
    }

    const mark = e.target.closest('button')?.getAttribute('data-mark');
    if (!mark) {
      return;
    }

    const markType = this.view.state.schema.marks[mark];
    toggleMark(markType)(this.view.state, this.view.dispatch);
  };

  public update(view: EditorView, lastState: null | EditorState) {
    const { state } = view;

    if (lastState && lastState.doc.eq(state.doc) && lastState.selection.eq(state.selection)) {
      return;
    }

    if (state.selection.empty) {
      this.tooltip.style.display = 'none';
      return;
    }

    this.tooltip.style.display = '';

    const { from, to } = state.selection;

    const start = view.coordsAtPos(from);
    const end = view.coordsAtPos(to);

    const box = this.tooltip.offsetParent!.getBoundingClientRect();

    const left = Math.max((start.left + end.left) / 2, start.left + 3);

    this.tooltip.style.left = `${left - box.left}px`;
    this.tooltip.style.bottom = `${box.bottom - start.top}px`;
  }

  public destroy() {
    this.tooltip.remove();
  }
}

export class RichTextBox {
  private readonly schema: Schema;
  private readonly state: EditorState;
  private readonly view: EditorView;

  public constructor(
    private readonly element: HTMLElement,
    private readonly changeCallback?: (value: string) => void,
  ) {
    this.schema = new Schema({
      nodes: {
        doc: { content: 'block+' },
        paragraph: {
          content: 'text*',
          group: 'block',
          marks: '_',
          toDOM() { return ['p', 0] },
          parseDOM: [{ tag: 'p' }],
        },
        text: { group: 'inline' },
      },
      marks: {
        bold: {
          toDOM() { return ['strong', { class: 'markup markup_bold' }] },
          parseDOM: [{ tag: 'strong' }, { tag: '.markup_bold' }],
        },
        italic: {
          toDOM() { return ['em', { class: 'markup markup_italic' }] },
          parseDOM: [{ tag: 'em' }, { tag: '.markup_italic' }],
        },
        underline: {
          toDOM() { return ['span', { class: 'markup markup_underline' }] },
          parseDOM: [{ tag: '.markup_underline' }],
        },
        strike: {
          toDOM() { return ['del', { class: 'markup markup_strike' }] },
          parseDOM: [{ tag: 'del' }, { tag: '.markup_strike' }],
        },
        superscript: {
          toDOM() { return ['sup', { class: 'markup markup_superscript' }] },
          parseDOM: [{ tag: 'sup' }, { tag: '.markup_superscript' }],
        },
        subscript: {
          toDOM() { return ['sub', { class: 'markup markup_subscript' }] },
          parseDOM: [{ tag: 'sub' }, { tag: '.markup_subscript' }],
        },
        code: {
          toDOM() { return ['pre', { class: 'markup markup_code' }, 0] },
          parseDOM: [{ tag: '.markup_code' }],
        },
        codeblock: {
          toDOM() { return ['pre', { class: 'markup markup_codeblock' }, 0] },
          parseDOM: [{ tag: 'pre' }, { tag: '.markup_codeblock' }],
        },
        spoiler: {
          toDOM() { return ['span', { class: 'markup markup_spoiler' }] },
          parseDOM: [{ tag: '.markup_spoiler' }],
        },
      },
    });

    function toString(node: any): string {
      switch (node.type) {
        case 'doc':
          if (!node.content) {
            return '';
          }

          return node.content.map(toString).join('\n');

        case 'paragraph':
          if (!node.content) {
            return '';
          }

          return node.content.map(toString).join('');

        case 'text':
          let text = node.text;
          if (node.marks) {
            node.marks.forEach((mark: any) => {
              switch (mark.type) {
                case 'bold':
                  text = `[b]${text}[/b]`;
                  break;

                case 'italic':
                  text = `[i]${text}[/i]`;
                  break;

                case 'underline':
                  text = `[u]${text}[/u]`;
                  break;

                case 'strike':
                  text = `[s]${text}[/s]`;
                  break;

                case 'superscript':
                  text = `[sup]${text}[/sup]`;
                  break;

                case 'subscript':
                  text = `[sub]${text}[/sub]`;
                  break;

                case 'code':
                  text = `[code]${text}[/code]`;
                  break;

                case 'codeblock':
                  text = `[codeblock]${text}[/codeblock]`;
                  break;

                case 'spoiler':
                  text = `[spoiler]${text}[/spoiler]`;
                  break;

                default:
                  break;
              }
            });
          }

          return text;

        default:
          return '';
      }
    }

    const self = this;
    const changeTrack = new Plugin({
      state: {
        init(_, instance) { return {}; },
        apply(tr, state) {
          if (self.changeCallback && tr.docChanged) {
            self.changeCallback(toString(tr.doc.toJSON()));
          }

          return state;
        },
      },
    });

    const tooltip = new Plugin({
      view(editorView) { return new Tooltip(editorView) }
    });

    this.state = EditorState.create({
      doc: DOMParser.fromSchema(this.schema).parse(element),
      plugins: [
        history(),
        keymap({
          'Mod-z': undo,
          'Mod-y': redo,
          'Alt-b': toggleMark(this.schema.marks.bold),
          'Alt-i': toggleMark(this.schema.marks.italic),
          'Alt-u': toggleMark(this.schema.marks.underline),
          'Alt-t': toggleMark(this.schema.marks.strike),
          'Alt-c': toggleMark(this.schema.marks.code),
          'Alt-p': toggleMark(this.schema.marks.spoiler),
        }),
        keymap(baseKeymap),
        changeTrack,
        tooltip,
      ],
    });

    this.view = new EditorView(element, {
      state: this.state,
    });
  }

  public clear() {
    selectAll(this.view.state, this.view.dispatch);
    deleteSelection(this.view.state, this.view.dispatch);
  }

  public focus() {
    this.view.focus();
  }
}

export default RichTextBox;
