/* cSpell:disable */
// Note: this file is auto concatenated to the end of the bundled d.ts during
// build.

// This code is based on react definition in DefinitelyTyped published under the MIT license.
//      Repository: https://github.com/DefinitelyTyped/DefinitelyTyped
//      Path in the repository: types/react/index.d.ts
//
// Copyrights of original definition are:
//      AssureSign <http://www.assuresign.com>
//      Microsoft <https://microsoft.com>
//                 John Reilly <https://github.com/johnnyreilly>
//      Benoit Benezech <https://github.com/bbenezech>
//      Patricio Zavolinsky <https://github.com/pzavolinsky>
//      Digiguru <https://github.com/digiguru>
//      Eric Anderson <https://github.com/ericanderson>
//      Dovydas Navickas <https://github.com/DovydasNavickas>
//                 Josh Rutherford <https://github.com/theruther4d>
//                 Guilherme Hübner <https://github.com/guilhermehubner>
//                 Ferdy Budhidharma <https://github.com/ferdaber>
//                 Johann Rakotoharisoa <https://github.com/jrakotoharisoa>
//                 Olivier Pascal <https://github.com/pascaloliv>
//                 Martin Hochel <https://github.com/hotell>
//                 Frank Li <https://github.com/franklixuefei>
//                 Jessica Franco <https://github.com/Jessidhia>
//                 Saransh Kataria <https://github.com/saranshkataria>
//                 Kanitkorn Sujautra <https://github.com/lukyth>
//                 Sebastian Silbermann <https://github.com/eps1lon>

import type * as CSS from 'csstype'

export interface CSSProperties
  extends CSS.Properties<string | number>,
    CSS.PropertiesHyphen<string | number> {
  /**
   * The index signature was removed to enable closed typing for style
   * using CSSType. You're able to use type assertion or module augmentation
   * to add properties or an index signature of your own.
   *
   * For examples and more information, visit:
   * https://github.com/frenic/csstype#what-should-i-do-when-i-get-type-errors
   */
  [v: `--${string}`]: string | number | undefined
}

type Booleanish = boolean | 'true' | 'false'
type Numberish = number | string

interface HTMLWebViewElement extends HTMLElement {}

// All the WAI-ARIA 1.1 attributes from https://www.w3.org/TR/wai-aria-1.1/
export interface AriaAttributes {
  /** Identifies the currently active element when DOM focus is on a composite widget, textbox, group, or application. */
  'aria-activedescendant'?: string | undefined
  /** Indicates whether assistive technologies will present all, or only parts of, the changed region based on the change notifications defined by the aria-relevant attribute. */
  'aria-atomic'?: Booleanish | undefined
  /**
   * Indicates whether inputting text could trigger display of one or more predictions of the user's intended value for an input and specifies how predictions would be
   * presented if they are made.
   */
  'aria-autocomplete'?: 'none' | 'inline' | 'list' | 'both' | undefined
  /** Indicates an element is being modified and that assistive technologies MAY want to wait until the modifications are complete before exposing them to the user. */
  'aria-busy'?: Booleanish | undefined
  /**
   * Indicates the current "checked" state of checkboxes, radio buttons, and other widgets.
   * @see aria-pressed @see aria-selected.
   */
  'aria-checked'?: Booleanish | 'mixed' | undefined
  /**
   * Defines the total number of columns in a table, grid, or treegrid.
   * @see aria-colindex.
   */
  'aria-colcount'?: Numberish | undefined
  /**
   * Defines an element's column index or position with respect to the total number of columns within a table, grid, or treegrid.
   * @see aria-colcount @see aria-colspan.
   */
  'aria-colindex'?: Numberish | undefined
  /**
   * Defines the number of columns spanned by a cell or gridcell within a table, grid, or treegrid.
   * @see aria-colindex @see aria-rowspan.
   */
  'aria-colspan'?: Numberish | undefined
  /**
   * Identifies the element (or elements) whose contents or presence are controlled by the current element.
   * @see aria-owns.
   */
  'aria-controls'?: string | undefined
  /** Indicates the element that represents the current item within a container or set of related elements. */
  'aria-current'?:
    | Booleanish
    | 'page'
    | 'step'
    | 'location'
    | 'date'
    | 'time'
    | undefined
  /**
   * Identifies the element (or elements) that describes the object.
   * @see aria-labelledby
   */
  'aria-describedby'?: string | undefined
  /**
   * Identifies the element that provides a detailed, extended description for the object.
   * @see aria-describedby.
   */
  'aria-details'?: string | undefined
  /**
   * Indicates that the element is perceivable but disabled, so it is not editable or otherwise operable.
   * @see aria-hidden @see aria-readonly.
   */
  'aria-disabled'?: Booleanish | undefined
  /**
   * Indicates what functions can be performed when a dragged object is released on the drop target.
   * @deprecated in ARIA 1.1
   */
  'aria-dropeffect'?:
    | 'none'
    | 'copy'
    | 'execute'
    | 'link'
    | 'move'
    | 'popup'
    | undefined
  /**
   * Identifies the element that provides an error message for the object.
   * @see aria-invalid @see aria-describedby.
   */
  'aria-errormessage'?: string | undefined
  /** Indicates whether the element, or another grouping element it controls, is currently expanded or collapsed. */
  'aria-expanded'?: Booleanish | undefined
  /**
   * Identifies the next element (or elements) in an alternate reading order of content which, at the user's discretion,
   * allows assistive technology to override the general default of reading in document source order.
   */
  'aria-flowto'?: string | undefined
  /**
   * Indicates an element's "grabbed" state in a drag-and-drop operation.
   * @deprecated in ARIA 1.1
   */
  'aria-grabbed'?: Booleanish | undefined
  /** Indicates the availability and type of interactive popup element, such as menu or dialog, that can be triggered by an element. */
  'aria-haspopup'?:
    | Booleanish
    | 'menu'
    | 'listbox'
    | 'tree'
    | 'grid'
    | 'dialog'
    | undefined
  /**
   * Indicates whether the element is exposed to an accessibility API.
   * @see aria-disabled.
   */
  'aria-hidden'?: Booleanish | undefined
  /**
   * Indicates the entered value does not conform to the format expected by the application.
   * @see aria-errormessage.
   */
  'aria-invalid'?: Booleanish | 'grammar' | 'spelling' | undefined
  /** Indicates keyboard shortcuts that an author has implemented to activate or give focus to an element. */
  'aria-keyshortcuts'?: string | undefined
  /**
   * Defines a string value that labels the current element.
   * @see aria-labelledby.
   */
  'aria-label'?: string | undefined
  /**
   * Identifies the element (or elements) that labels the current element.
   * @see aria-describedby.
   */
  'aria-labelledby'?: string | undefined
  /** Defines the hierarchical level of an element within a structure. */
  'aria-level'?: Numberish | undefined
  /** Indicates that an element will be updated, and describes the types of updates the user agents, assistive technologies, and user can expect from the live region. */
  'aria-live'?: 'off' | 'assertive' | 'polite' | undefined
  /** Indicates whether an element is modal when displayed. */
  'aria-modal'?: Booleanish | undefined
  /** Indicates whether a text box accepts multiple lines of input or only a single line. */
  'aria-multiline'?: Booleanish | undefined
  /** Indicates that the user may select more than one item from the current selectable descendants. */
  'aria-multiselectable'?: Booleanish | undefined
  /** Indicates whether the element's orientation is horizontal, vertical, or unknown/ambiguous. */
  'aria-orientation'?: 'horizontal' | 'vertical' | undefined
  /**
   * Identifies an element (or elements) in order to define a visual, functional, or contextual parent/child relationship
   * between DOM elements where the DOM hierarchy cannot be used to represent the relationship.
   * @see aria-controls.
   */
  'aria-owns'?: string | undefined
  /**
   * Defines a short hint (a word or short phrase) intended to aid the user with data entry when the control has no value.
   * A hint could be a sample value or a brief description of the expected format.
   */
  'aria-placeholder'?: string | undefined
  /**
   * Defines an element's number or position in the current set of listitems or treeitems. Not required if all elements in the set are present in the DOM.
   * @see aria-setsize.
   */
  'aria-posinset'?: Numberish | undefined
  /**
   * Indicates the current "pressed" state of toggle buttons.
   * @see aria-checked @see aria-selected.
   */
  'aria-pressed'?: Booleanish | 'mixed' | undefined
  /**
   * Indicates that the element is not editable, but is otherwise operable.
   * @see aria-disabled.
   */
  'aria-readonly'?: Booleanish | undefined
  /**
   * Indicates what notifications the user agent will trigger when the accessibility tree within a live region is modified.
   * @see aria-atomic.
   */
  'aria-relevant'?:
    | 'additions'
    | 'additions removals'
    | 'additions text'
    | 'all'
    | 'removals'
    | 'removals additions'
    | 'removals text'
    | 'text'
    | 'text additions'
    | 'text removals'
    | undefined
  /** Indicates that user input is required on the element before a form may be submitted. */
  'aria-required'?: Booleanish | undefined
  /** Defines a human-readable, author-localized description for the role of an element. */
  'aria-roledescription'?: string | undefined
  /**
   * Defines the total number of rows in a table, grid, or treegrid.
   * @see aria-rowindex.
   */
  'aria-rowcount'?: Numberish | undefined
  /**
   * Defines an element's row index or position with respect to the total number of rows within a table, grid, or treegrid.
   * @see aria-rowcount @see aria-rowspan.
   */
  'aria-rowindex'?: Numberish | undefined
  /**
   * Defines the number of rows spanned by a cell or gridcell within a table, grid, or treegrid.
   * @see aria-rowindex @see aria-colspan.
   */
  'aria-rowspan'?: Numberish | undefined
  /**
   * Indicates the current "selected" state of various widgets.
   * @see aria-checked @see aria-pressed.
   */
  'aria-selected'?: Booleanish | undefined
  /**
   * Defines the number of items in the current set of listitems or treeitems. Not required if all elements in the set are present in the DOM.
   * @see aria-posinset.
   */
  'aria-setsize'?: Numberish | undefined
  /** Indicates if items in a table or grid are sorted in ascending or descending order. */
  'aria-sort'?: 'none' | 'ascending' | 'descending' | 'other' | undefined
  /** Defines the maximum allowed value for a range widget. */
  'aria-valuemax'?: Numberish | undefined
  /** Defines the minimum allowed value for a range widget. */
  'aria-valuemin'?: Numberish | undefined
  /**
   * Defines the current value for a range widget.
   * @see aria-valuetext.
   */
  'aria-valuenow'?: Numberish | undefined
  /** Defines the human readable text alternative of aria-valuenow for a range widget. */
  'aria-valuetext'?: string | undefined
}

/**
 * @see {@link https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/crossorigin MDN}
 */
type CrossOrigin = 'anonymous' | 'use-credentials' | ''

// Vue's style normalization supports nested arrays
export type StyleValue =
  | false
  | null
  | undefined
  | string
  | CSSProperties
  | Array<StyleValue>

export interface HTMLAttributes<T = HTMLElement>
  extends AriaAttributes,
    EventHandlers<Events<T>> {
  innerHTML?: string | undefined

  class?: any | undefined
  style?: StyleValue | undefined

  // Standard HTML Attributes
  accesskey?: string | undefined
  autocapitalize?:
    | 'off'
    | 'none'
    | 'on'
    | 'sentences'
    | 'words'
    | 'characters'
    | undefined
    | (string & {})
  autofocus?: Booleanish | undefined
  contenteditable?: Booleanish | 'inherit' | 'plaintext-only' | undefined
  contextmenu?: string | undefined
  dir?: string | undefined
  draggable?: Booleanish | undefined
  enterKeyHint?:
    | 'enter'
    | 'done'
    | 'go'
    | 'next'
    | 'previous'
    | 'search'
    | 'send'
    | undefined
  hidden?: Booleanish | '' | 'hidden' | 'until-found' | undefined
  id?: string | undefined
  inert?: Booleanish | undefined
  lang?: string | undefined
  nonce?: string | undefined
  placeholder?: string | undefined
  spellcheck?: Booleanish | undefined
  tabindex?: Numberish | undefined
  title?: string | undefined
  translate?: 'yes' | 'no' | undefined

  // Unknown
  radiogroup?: string | undefined // <command>, <menuitem>

  // WAI-ARIA
  role?: string | undefined

  // RDFa Attributes
  about?: string | undefined
  content?: string | undefined
  datatype?: string | undefined
  inlist?: any | undefined
  prefix?: string | undefined
  property?: string | undefined
  rel?: string | undefined
  resource?: string | undefined
  rev?: string | undefined
  typeof?: string | undefined
  vocab?: string | undefined

  // Non-standard Attributes
  autocorrect?: string | undefined
  autosave?: string | undefined
  color?: string | undefined
  itemprop?: string | undefined
  itemscope?: Booleanish | undefined
  itemtype?: string | undefined
  itemid?: string | undefined
  itemref?: string | undefined
  results?: Numberish | undefined
  security?: string | undefined
  unselectable?: 'on' | 'off' | undefined

  // Living Standard
  /**
   * Hints at the type of data that might be entered by the user while editing the element or its contents
   * @see https://html.spec.whatwg.org/multipage/interaction.html#input-modalities:-the-inputmode-attribute
   */
  inputmode?:
    | 'none'
    | 'text'
    | 'tel'
    | 'url'
    | 'email'
    | 'numeric'
    | 'decimal'
    | 'search'
    | undefined
  /**
   * Specify that a standard HTML element should behave like a defined custom built-in element
   * @see https://html.spec.whatwg.org/multipage/custom-elements.html#attr-is
   */
  is?: string | undefined
}

type HTMLAttributeReferrerPolicy =
  | ''
  | 'no-referrer'
  | 'no-referrer-when-downgrade'
  | 'origin'
  | 'origin-when-cross-origin'
  | 'same-origin'
  | 'strict-origin'
  | 'strict-origin-when-cross-origin'
  | 'unsafe-url'

export interface AnchorHTMLAttributes<T> extends HTMLAttributes<T> {
  download?: any | undefined
  href?: string | undefined
  hreflang?: string | undefined
  media?: string | undefined
  ping?: string | undefined
  rel?: string | undefined
  target?: string | undefined
  type?: string | undefined
  referrerpolicy?: HTMLAttributeReferrerPolicy | undefined
}

export interface AreaHTMLAttributes<T> extends HTMLAttributes<T> {
  alt?: string | undefined
  coords?: string | undefined
  download?: any | undefined
  href?: string | undefined
  hreflang?: string | undefined
  media?: string | undefined
  referrerpolicy?: HTMLAttributeReferrerPolicy | undefined
  shape?: string | undefined
  target?: string | undefined
}

export interface AudioHTMLAttributes<T> extends MediaHTMLAttributes<T> {}

export interface BaseHTMLAttributes<T> extends HTMLAttributes<T> {
  href?: string | undefined
  target?: string | undefined
}

export interface BlockquoteHTMLAttributes<T> extends HTMLAttributes<T> {
  cite?: string | undefined
}

export interface ButtonHTMLAttributes<T> extends HTMLAttributes<T> {
  disabled?: Booleanish | undefined
  form?: string | undefined
  formaction?: string | undefined
  formenctype?: string | undefined
  formmethod?: string | undefined
  formnovalidate?: Booleanish | undefined
  formtarget?: string | undefined
  name?: string | undefined
  type?: 'submit' | 'reset' | 'button' | undefined
  value?: string | ReadonlyArray<string> | number | undefined
}

export interface CanvasHTMLAttributes<T> extends HTMLAttributes<T> {
  height?: Numberish | undefined
  width?: Numberish | undefined
}

export interface ColHTMLAttributes<T> extends HTMLAttributes<T> {
  span?: Numberish | undefined
  width?: Numberish | undefined
}

export interface ColgroupHTMLAttributes<T> extends HTMLAttributes<T> {
  span?: Numberish | undefined
}

export interface DataHTMLAttributes<T> extends HTMLAttributes<T> {
  value?: string | ReadonlyArray<string> | number | undefined
}

export interface DetailsHTMLAttributes<T> extends HTMLAttributes<T> {
  name?: string | undefined
  open?: Booleanish | undefined
}

export interface DelHTMLAttributes<T> extends HTMLAttributes<T> {
  cite?: string | undefined
  datetime?: string | undefined
}

export interface DialogHTMLAttributes<T> extends HTMLAttributes<T> {
  onCancel?: EventHandler<SyntheticEvent<T>> | undefined
  onClose?: EventHandler<SyntheticEvent<T>> | undefined
  open?: boolean | undefined
}

export interface EmbedHTMLAttributes<T> extends HTMLAttributes<T> {
  height?: Numberish | undefined
  src?: string | undefined
  type?: string | undefined
  width?: Numberish | undefined
}

export interface FieldsetHTMLAttributes<T> extends HTMLAttributes<T> {
  disabled?: Booleanish | undefined
  form?: string | undefined
  name?: string | undefined
}

export interface FormHTMLAttributes<T> extends HTMLAttributes<T> {
  acceptcharset?: string | undefined
  action?: string | undefined
  autocomplete?: string | undefined
  enctype?: string | undefined
  method?: string | undefined
  name?: string | undefined
  novalidate?: Booleanish | undefined
  target?: string | undefined
}

export interface HtmlHTMLAttributes<T> extends HTMLAttributes<T> {
  manifest?: string | undefined
}

export interface IframeHTMLAttributes<T> extends HTMLAttributes<T> {
  allow?: string | undefined
  allowfullscreen?: Booleanish | undefined
  allowtransparency?: Booleanish | undefined
  /** @deprecated */
  frameborder?: Numberish | undefined
  height?: Numberish | undefined
  loading?: 'eager' | 'lazy' | undefined
  /** @deprecated */
  marginheight?: Numberish | undefined
  /** @deprecated */
  marginwidth?: Numberish | undefined
  name?: string | undefined
  referrerpolicy?: HTMLAttributeReferrerPolicy | undefined
  sandbox?: string | undefined
  /** @deprecated */
  scrolling?: string | undefined
  seamless?: Booleanish | undefined
  src?: string | undefined
  srcdoc?: string | undefined
  width?: Numberish | undefined
}

export interface ImgHTMLAttributes<T> extends HTMLAttributes<T> {
  alt?: string | undefined
  crossorigin?: CrossOrigin | undefined
  decoding?: 'async' | 'auto' | 'sync' | undefined
  height?: Numberish | undefined
  loading?: 'eager' | 'lazy' | undefined
  referrerpolicy?: HTMLAttributeReferrerPolicy | undefined
  sizes?: string | undefined
  src?: string | undefined
  srcset?: string | undefined
  usemap?: string | undefined
  width?: Numberish | undefined
}

export interface InsHTMLAttributes<T> extends HTMLAttributes<T> {
  cite?: string | undefined
  datetime?: string | undefined
}

export type InputTypeHTMLAttribute =
  | 'button'
  | 'checkbox'
  | 'color'
  | 'date'
  | 'datetime-local'
  | 'email'
  | 'file'
  | 'hidden'
  | 'image'
  | 'month'
  | 'number'
  | 'password'
  | 'radio'
  | 'range'
  | 'reset'
  | 'search'
  | 'submit'
  | 'tel'
  | 'text'
  | 'time'
  | 'url'
  | 'week'
  | (string & {})

type AutoFillAddressKind = 'billing' | 'shipping'
type AutoFillBase = '' | 'off' | 'on'
type AutoFillContactField =
  | 'email'
  | 'tel'
  | 'tel-area-code'
  | 'tel-country-code'
  | 'tel-extension'
  | 'tel-local'
  | 'tel-local-prefix'
  | 'tel-local-suffix'
  | 'tel-national'
type AutoFillContactKind = 'home' | 'mobile' | 'work'
type AutoFillCredentialField = 'webauthn'
type AutoFillNormalField =
  | 'additional-name'
  | 'address-level1'
  | 'address-level2'
  | 'address-level3'
  | 'address-level4'
  | 'address-line1'
  | 'address-line2'
  | 'address-line3'
  | 'bday-day'
  | 'bday-month'
  | 'bday-year'
  | 'cc-csc'
  | 'cc-exp'
  | 'cc-exp-month'
  | 'cc-exp-year'
  | 'cc-family-name'
  | 'cc-given-name'
  | 'cc-name'
  | 'cc-number'
  | 'cc-type'
  | 'country'
  | 'country-name'
  | 'current-password'
  | 'family-name'
  | 'given-name'
  | 'honorific-prefix'
  | 'honorific-suffix'
  | 'name'
  | 'new-password'
  | 'one-time-code'
  | 'organization'
  | 'postal-code'
  | 'street-address'
  | 'transaction-amount'
  | 'transaction-currency'
  | 'username'
type OptionalPrefixToken<T extends string> = `${T} ` | ''
type OptionalPostfixToken<T extends string> = ` ${T}` | ''
type AutoFillField =
  | AutoFillNormalField
  | `${OptionalPrefixToken<AutoFillContactKind>}${AutoFillContactField}`
type AutoFillSection = `section-${string}`
type AutoFill =
  | AutoFillBase
  | `${OptionalPrefixToken<AutoFillSection>}${OptionalPrefixToken<AutoFillAddressKind>}${AutoFillField}${OptionalPostfixToken<AutoFillCredentialField>}`
type HTMLInputAutoCompleteAttribute = AutoFill | (string & {})

export interface InputHTMLAttributes<T> extends HTMLAttributes<T> {
  accept?: string | undefined
  alt?: string | undefined
  autocomplete?: HTMLInputAutoCompleteAttribute | undefined
  capture?: boolean | 'user' | 'environment' // https://www.w3.org/tr/html-media-capture/#the-capture-attribute | undefined
  checked?: Booleanish | any[] | Set<any> // for IDE v-model multi-checkbox support | undefined
  disabled?: Booleanish | undefined
  form?: string | undefined
  formaction?: string | undefined
  formenctype?: string | undefined
  formmethod?: string | undefined
  formnovalidate?: Booleanish | undefined
  formtarget?: string | undefined
  height?: Numberish | undefined
  indeterminate?: boolean | undefined
  list?: string | undefined
  max?: Numberish | undefined
  maxlength?: Numberish | undefined
  min?: Numberish | undefined
  minlength?: Numberish | undefined
  multiple?: Booleanish | undefined
  name?: string | undefined
  pattern?: string | undefined
  placeholder?: string | undefined
  readonly?: Booleanish | undefined
  required?: Booleanish | undefined
  size?: Numberish | undefined
  src?: string | undefined
  step?: Numberish | undefined
  type?: InputTypeHTMLAttribute | undefined
  value?: any // we support :value to be bound to anything w/ v-model | undefined
  width?: Numberish | undefined
}

export interface KeygenHTMLAttributes<T> extends HTMLAttributes<T> {
  challenge?: string | undefined
  disabled?: Booleanish | undefined
  form?: string | undefined
  keytype?: string | undefined
  keyparams?: string | undefined
  name?: string | undefined
}

export interface LabelHTMLAttributes<T> extends HTMLAttributes<T> {
  for?: string | undefined
  form?: string | undefined
}

export interface LiHTMLAttributes<T> extends HTMLAttributes<T> {
  value?: string | ReadonlyArray<string> | number | undefined
}

export interface LinkHTMLAttributes<T> extends HTMLAttributes<T> {
  as?: string | undefined
  crossorigin?: CrossOrigin | undefined
  fetchPriority?: 'high' | 'low' | 'auto' | undefined
  href?: string | undefined
  hreflang?: string | undefined
  integrity?: string | undefined
  media?: string | undefined
  imageSrcSet?: string | undefined
  imageSizes?: string | undefined
  referrerpolicy?: HTMLAttributeReferrerPolicy | undefined
  sizes?: string | undefined
  type?: string | undefined
  charset?: string | undefined
}

export interface MapHTMLAttributes<T> extends HTMLAttributes<T> {
  name?: string | undefined
}

export interface MenuHTMLAttributes<T> extends HTMLAttributes<T> {
  type?: string | undefined
}

export interface MediaHTMLAttributes<T> extends HTMLAttributes<T> {
  autoplay?: Booleanish | undefined
  controls?: Booleanish | undefined
  controlslist?: string | undefined
  crossorigin?: CrossOrigin | undefined
  loop?: Booleanish | undefined
  mediagroup?: string | undefined
  muted?: Booleanish | undefined
  playsinline?: Booleanish | undefined
  preload?: string | undefined
  src?: string | undefined
}

export interface MetaHTMLAttributes<T> extends HTMLAttributes<T> {
  charset?: string | undefined
  content?: string | undefined
  httpequiv?: string | undefined
  media?: string | undefined | undefined
  name?: string | undefined
}

export interface MeterHTMLAttributes<T> extends HTMLAttributes<T> {
  form?: string | undefined
  high?: Numberish | undefined
  low?: Numberish | undefined
  max?: Numberish | undefined
  min?: Numberish | undefined
  optimum?: Numberish | undefined
  value?: string | ReadonlyArray<string> | number | undefined
}

export interface QuoteHTMLAttributes<T> extends HTMLAttributes<T> {
  cite?: string | undefined
}

export interface ObjectHTMLAttributes<T> extends HTMLAttributes<T> {
  classid?: string | undefined
  data?: string | undefined
  form?: string | undefined
  height?: Numberish | undefined
  name?: string | undefined
  type?: string | undefined
  usemap?: string | undefined
  width?: Numberish | undefined
  wmode?: string | undefined
}

export interface OlHTMLAttributes<T> extends HTMLAttributes<T> {
  reversed?: Booleanish | undefined
  start?: Numberish | undefined
  type?: '1' | 'a' | 'A' | 'i' | 'I' | undefined
}

export interface OptgroupHTMLAttributes<T> extends HTMLAttributes<T> {
  disabled?: Booleanish | undefined
  label?: string | undefined
}

export interface OptionHTMLAttributes<T> extends HTMLAttributes<T> {
  disabled?: Booleanish | undefined
  label?: string | undefined
  selected?: Booleanish | undefined
  value?: any // we support :value to be bound to anything w/ v-model | undefined
}

export interface OutputHTMLAttributes<T> extends HTMLAttributes<T> {
  for?: string | undefined
  form?: string | undefined
  name?: string | undefined
}

export interface ParamHTMLAttributes<T> extends HTMLAttributes<T> {
  name?: string | undefined
  value?: string | ReadonlyArray<string> | number | undefined
}

export interface ProgressHTMLAttributes<T> extends HTMLAttributes<T> {
  max?: Numberish | undefined
  value?: string | ReadonlyArray<string> | number | undefined
}

export interface ScriptHTMLAttributes<T> extends HTMLAttributes<T> {
  async?: Booleanish | undefined
  /** @deprecated */
  charset?: string | undefined
  crossorigin?: CrossOrigin | undefined
  defer?: Booleanish | undefined
  integrity?: string | undefined
  nomodule?: Booleanish | undefined
  referrerpolicy?: HTMLAttributeReferrerPolicy | undefined
  src?: string | undefined
  type?: string | undefined
}

export interface SelectHTMLAttributes<T> extends HTMLAttributes<T> {
  autocomplete?: string | undefined
  disabled?: Booleanish | undefined
  form?: string | undefined
  multiple?: Booleanish | undefined
  name?: string | undefined
  required?: Booleanish | undefined
  size?: Numberish | undefined
  value?: any // we support :value to be bound to anything w/ v-model | undefined
}

export interface SourceHTMLAttributes<T> extends HTMLAttributes<T> {
  height?: number | undefined
  media?: string | undefined
  sizes?: string | undefined
  src?: string | undefined
  srcset?: string | undefined
  type?: string | undefined
  width?: number | undefined
}

export interface StyleHTMLAttributes<T> extends HTMLAttributes<T> {
  media?: string | undefined
  scoped?: Booleanish | undefined
  type?: string | undefined
}

export interface TableHTMLAttributes<T> extends HTMLAttributes<T> {
  align?: 'left' | 'center' | 'right' | undefined
  bgcolor?: string | undefined
  border?: number | undefined
  cellpadding?: Numberish | undefined
  cellspacing?: Numberish | undefined
  frame?: Booleanish | undefined
  rules?: 'none' | 'groups' | 'rows' | 'columns' | 'all' | undefined
  summary?: string | undefined
  width?: Numberish | undefined
}

export interface TextareaHTMLAttributes<T> extends HTMLAttributes<T> {
  autocomplete?: string | undefined
  cols?: Numberish | undefined
  dirname?: string | undefined
  disabled?: Booleanish | undefined
  form?: string | undefined
  maxlength?: Numberish | undefined
  minlength?: Numberish | undefined
  name?: string | undefined
  placeholder?: string | undefined
  readonly?: Booleanish | undefined
  required?: Booleanish | undefined
  rows?: Numberish | undefined
  value?: string | ReadonlyArray<string> | number | null | undefined
  wrap?: string | undefined
}

export interface TdHTMLAttributes<T> extends HTMLAttributes<T> {
  align?: 'left' | 'center' | 'right' | 'justify' | 'char' | undefined
  colspan?: Numberish | undefined
  headers?: string | undefined
  rowspan?: Numberish | undefined
  scope?: string | undefined
  abbr?: string | undefined
  height?: Numberish | undefined
  width?: Numberish | undefined
  valign?: 'top' | 'middle' | 'bottom' | 'baseline' | undefined
}

export interface ThHTMLAttributes<T> extends HTMLAttributes<T> {
  align?: 'left' | 'center' | 'right' | 'justify' | 'char' | undefined
  colspan?: Numberish | undefined
  headers?: string | undefined
  rowspan?: Numberish | undefined
  scope?: string | undefined
  abbr?: string | undefined
}

export interface TimeHTMLAttributes<T> extends HTMLAttributes<T> {
  datetime?: string | undefined
}

export interface TrackHTMLAttributes<T> extends HTMLAttributes<T> {
  default?: Booleanish | undefined
  kind?: string | undefined
  label?: string | undefined
  src?: string | undefined
  srclang?: string | undefined
}

export interface VideoHTMLAttributes<T> extends MediaHTMLAttributes<T> {
  height?: Numberish | undefined
  playsinline?: Booleanish | undefined
  poster?: string | undefined
  width?: Numberish | undefined
  disablePictureInPicture?: Booleanish | undefined
  disableRemotePlayback?: Booleanish | undefined
}

export interface WebViewHTMLAttributes<T> extends HTMLAttributes<T> {
  allowfullscreen?: Booleanish | undefined
  allowpopups?: Booleanish | undefined
  autosize?: Booleanish | undefined
  blinkfeatures?: string | undefined
  disableblinkfeatures?: string | undefined
  disableguestresize?: Booleanish | undefined
  disablewebsecurity?: Booleanish | undefined
  guestinstance?: string | undefined
  httpreferrer?: string | undefined
  nodeintegration?: Booleanish | undefined
  partition?: string | undefined
  plugins?: Booleanish | undefined
  preload?: string | undefined
  src?: string | undefined
  useragent?: string | undefined
  webpreferences?: string | undefined
}

export interface SVGAttributes extends AriaAttributes, EventHandlers<Events> {
  innerHTML?: string | undefined

  /**
   * SVG Styling Attributes
   * @see https://www.w3.org/TR/SVG/styling.html#ElementSpecificStyling
   */
  class?: any | undefined
  style?: StyleValue | undefined

  color?: string | undefined
  height?: Numberish | undefined
  id?: string | undefined
  lang?: string | undefined
  max?: Numberish | undefined
  media?: string | undefined
  method?: string | undefined
  min?: Numberish | undefined
  name?: string | undefined
  target?: string | undefined
  type?: string | undefined
  width?: Numberish | undefined

  // Other HTML properties supported by SVG elements in browsers
  role?: string | undefined
  tabindex?: Numberish | undefined
  crossOrigin?: CrossOrigin | undefined

  // SVG Specific attributes
  'accent-height'?: Numberish | undefined
  accumulate?: 'none' | 'sum' | undefined
  additive?: 'replace' | 'sum' | undefined
  'alignment-baseline'?:
    | 'auto'
    | 'baseline'
    | 'before-edge'
    | 'text-before-edge'
    | 'middle'
    | 'central'
    | 'after-edge'
    | 'text-after-edge'
    | 'ideographic'
    | 'alphabetic'
    | 'hanging'
    | 'mathematical'
    | 'inherit'
    | undefined
  allowReorder?: 'no' | 'yes' | undefined
  alphabetic?: Numberish | undefined
  amplitude?: Numberish | undefined
  'arabic-form'?: 'initial' | 'medial' | 'terminal' | 'isolated' | undefined
  ascent?: Numberish | undefined
  attributeName?: string | undefined
  attributeType?: string | undefined
  autoReverse?: Numberish | undefined
  azimuth?: Numberish | undefined
  baseFrequency?: Numberish | undefined
  'baseline-shift'?: Numberish | undefined
  baseProfile?: Numberish | undefined
  bbox?: Numberish | undefined
  begin?: Numberish | undefined
  bias?: Numberish | undefined
  by?: Numberish | undefined
  calcMode?: Numberish | undefined
  'cap-height'?: Numberish | undefined
  clip?: Numberish | undefined
  'clip-path'?: string | undefined
  clipPathUnits?: Numberish | undefined
  'clip-rule'?: Numberish | undefined
  'color-interpolation'?: Numberish | undefined
  'color-interpolation-filters'?:
    | 'auto'
    | 'sRGB'
    | 'linearRGB'
    | 'inherit'
    | undefined
  'color-profile'?: Numberish | undefined
  'color-rendering'?: Numberish | undefined
  contentScriptType?: Numberish | undefined
  contentStyleType?: Numberish | undefined
  cursor?: Numberish | undefined
  cx?: Numberish | undefined
  cy?: Numberish | undefined
  d?: string | undefined
  decelerate?: Numberish | undefined
  descent?: Numberish | undefined
  diffuseConstant?: Numberish | undefined
  direction?: Numberish | undefined
  display?: Numberish | undefined
  divisor?: Numberish | undefined
  'dominant-baseline'?: Numberish | undefined
  dur?: Numberish | undefined
  dx?: Numberish | undefined
  dy?: Numberish | undefined
  edgeMode?: Numberish | undefined
  elevation?: Numberish | undefined
  'enable-background'?: Numberish | undefined
  end?: Numberish | undefined
  exponent?: Numberish | undefined
  externalResourcesRequired?: Numberish | undefined
  fill?: string | undefined
  'fill-opacity'?: Numberish | undefined
  'fill-rule'?: 'nonzero' | 'evenodd' | 'inherit' | undefined
  filter?: string | undefined
  filterRes?: Numberish | undefined
  filterUnits?: Numberish | undefined
  'flood-color'?: Numberish | undefined
  'flood-opacity'?: Numberish | undefined
  focusable?: Numberish | undefined
  'font-family'?: string | undefined
  'font-size'?: Numberish | undefined
  'font-size-adjust'?: Numberish | undefined
  'font-stretch'?: Numberish | undefined
  'font-style'?: Numberish | undefined
  'font-variant'?: Numberish | undefined
  'font-weight'?: Numberish | undefined
  format?: Numberish | undefined
  from?: Numberish | undefined
  fx?: Numberish | undefined
  fy?: Numberish | undefined
  g1?: Numberish | undefined
  g2?: Numberish | undefined
  'glyph-name'?: Numberish | undefined
  'glyph-orientation-horizontal'?: Numberish | undefined
  'glyph-orientation-vertical'?: Numberish | undefined
  glyphRef?: Numberish | undefined
  gradientTransform?: string | undefined
  gradientUnits?: string | undefined
  hanging?: Numberish | undefined
  'horiz-adv-x'?: Numberish | undefined
  'horiz-origin-x'?: Numberish | undefined
  href?: string | undefined
  ideographic?: Numberish | undefined
  'image-rendering'?: Numberish | undefined
  in2?: Numberish | undefined
  in?: string | undefined
  intercept?: Numberish | undefined
  k1?: Numberish | undefined
  k2?: Numberish | undefined
  k3?: Numberish | undefined
  k4?: Numberish | undefined
  k?: Numberish | undefined
  kernelMatrix?: Numberish | undefined
  kernelUnitLength?: Numberish | undefined
  kerning?: Numberish | undefined
  keyPoints?: Numberish | undefined
  keySplines?: Numberish | undefined
  keyTimes?: Numberish | undefined
  lengthAdjust?: Numberish | undefined
  'letter-spacing'?: Numberish | undefined
  'lighting-color'?: Numberish | undefined
  limitingConeAngle?: Numberish | undefined
  local?: Numberish | undefined
  'marker-end'?: string | undefined
  markerHeight?: Numberish | undefined
  'marker-mid'?: string | undefined
  'marker-start'?: string | undefined
  markerUnits?: Numberish | undefined
  markerWidth?: Numberish | undefined
  mask?: string | undefined
  maskContentUnits?: Numberish | undefined
  maskUnits?: Numberish | undefined
  mathematical?: Numberish | undefined
  mode?: Numberish | undefined
  numOctaves?: Numberish | undefined
  offset?: Numberish | undefined
  opacity?: Numberish | undefined
  operator?: Numberish | undefined
  order?: Numberish | undefined
  orient?: Numberish | undefined
  orientation?: Numberish | undefined
  origin?: Numberish | undefined
  overflow?: Numberish | undefined
  'overline-position'?: Numberish | undefined
  'overline-thickness'?: Numberish | undefined
  'paint-order'?: Numberish | undefined
  'panose-1'?: Numberish | undefined
  pathLength?: Numberish | undefined
  patternContentUnits?: string | undefined
  patternTransform?: Numberish | undefined
  patternUnits?: string | undefined
  'pointer-events'?: Numberish | undefined
  points?: string | undefined
  pointsAtX?: Numberish | undefined
  pointsAtY?: Numberish | undefined
  pointsAtZ?: Numberish | undefined
  preserveAlpha?: Numberish | undefined
  preserveAspectRatio?: string | undefined
  primitiveUnits?: Numberish | undefined
  r?: Numberish | undefined
  radius?: Numberish | undefined
  refX?: Numberish | undefined
  refY?: Numberish | undefined
  renderingIntent?: Numberish | undefined
  repeatCount?: Numberish | undefined
  repeatDur?: Numberish | undefined
  requiredExtensions?: Numberish | undefined
  requiredFeatures?: Numberish | undefined
  restart?: Numberish | undefined
  result?: string | undefined
  rotate?: Numberish | undefined
  rx?: Numberish | undefined
  ry?: Numberish | undefined
  scale?: Numberish | undefined
  seed?: Numberish | undefined
  'shape-rendering'?: Numberish | undefined
  slope?: Numberish | undefined
  spacing?: Numberish | undefined
  specularConstant?: Numberish | undefined
  specularExponent?: Numberish | undefined
  speed?: Numberish | undefined
  spreadMethod?: string | undefined
  startOffset?: Numberish | undefined
  stdDeviation?: Numberish | undefined
  stemh?: Numberish | undefined
  stemv?: Numberish | undefined
  stitchTiles?: Numberish | undefined
  'stop-color'?: string | undefined
  'stop-opacity'?: Numberish | undefined
  'strikethrough-position'?: Numberish | undefined
  'strikethrough-thickness'?: Numberish | undefined
  string?: Numberish | undefined
  stroke?: string | undefined
  'stroke-dasharray'?: Numberish | undefined
  'stroke-dashoffset'?: Numberish | undefined
  'stroke-linecap'?: 'butt' | 'round' | 'square' | 'inherit' | undefined
  'stroke-linejoin'?: 'miter' | 'round' | 'bevel' | 'inherit' | undefined
  'stroke-miterlimit'?: Numberish | undefined
  'stroke-opacity'?: Numberish | undefined
  'stroke-width'?: Numberish | undefined
  surfaceScale?: Numberish | undefined
  systemLanguage?: Numberish | undefined
  tableValues?: Numberish | undefined
  targetX?: Numberish | undefined
  targetY?: Numberish | undefined
  'text-anchor'?: string | undefined
  'text-decoration'?: Numberish | undefined
  textLength?: Numberish | undefined
  'text-rendering'?: Numberish | undefined
  to?: Numberish | undefined
  transform?: string | undefined
  u1?: Numberish | undefined
  u2?: Numberish | undefined
  'underline-position'?: Numberish | undefined
  'underline-thickness'?: Numberish | undefined
  unicode?: Numberish | undefined
  'unicode-bidi'?: Numberish | undefined
  'unicode-range'?: Numberish | undefined
  'unitsPer-em'?: Numberish | undefined
  'v-alphabetic'?: Numberish | undefined
  values?: string | undefined
  'vector-effect'?: Numberish | undefined
  version?: string | undefined
  'vert-adv-y'?: Numberish | undefined
  'vert-origin-x'?: Numberish | undefined
  'vert-origin-y'?: Numberish | undefined
  'v-hanging'?: Numberish | undefined
  'v-ideographic'?: Numberish | undefined
  viewBox?: string | undefined
  viewTarget?: Numberish | undefined
  visibility?: Numberish | undefined
  'v-mathematical'?: Numberish | undefined
  widths?: Numberish | undefined
  'word-spacing'?: Numberish | undefined
  'writing-mode'?: Numberish | undefined
  x1?: Numberish | undefined
  x2?: Numberish | undefined
  x?: Numberish | undefined
  xChannelSelector?: string | undefined
  'x-height'?: Numberish | undefined
  xlinkActuate?: string | undefined
  xlinkArcrole?: string | undefined
  xlinkHref?: string | undefined
  xlinkRole?: string | undefined
  xlinkShow?: string | undefined
  xlinkTitle?: string | undefined
  xlinkType?: string | undefined
  xmlns?: string | undefined
  xmlnsXlink?: string | undefined
  y1?: Numberish | undefined
  y2?: Numberish | undefined
  y?: Numberish | undefined
  yChannelSelector?: string | undefined
  z?: Numberish | undefined
  zoomAndPan?: string | undefined
}

export interface IntrinsicElementAttributes {
  a: AnchorHTMLAttributes<HTMLAnchorElement>
  abbr: HTMLAttributes<HTMLElement>
  address: HTMLAttributes<HTMLElement>
  area: AreaHTMLAttributes<HTMLAreaElement>
  article: HTMLAttributes<HTMLElement>
  aside: HTMLAttributes<HTMLElement>
  audio: AudioHTMLAttributes<HTMLAudioElement>
  b: HTMLAttributes<HTMLElement>
  base: BaseHTMLAttributes<HTMLBaseElement>
  bdi: HTMLAttributes<HTMLElement>
  bdo: HTMLAttributes<HTMLElement>
  big: HTMLAttributes<HTMLElement>
  blockquote: BlockquoteHTMLAttributes<HTMLQuoteElement>
  body: HTMLAttributes<HTMLBodyElement>
  br: HTMLAttributes<HTMLBRElement>
  button: ButtonHTMLAttributes<HTMLButtonElement>
  canvas: CanvasHTMLAttributes<HTMLCanvasElement>
  caption: HTMLAttributes<HTMLElement>
  cite: HTMLAttributes<HTMLElement>
  code: HTMLAttributes<HTMLElement>
  col: ColHTMLAttributes<HTMLTableColElement>
  colgroup: ColgroupHTMLAttributes<HTMLTableColElement>
  data: DataHTMLAttributes<HTMLDataElement>
  datalist: HTMLAttributes<HTMLDataListElement>
  dd: HTMLAttributes<HTMLElement>
  del: DelHTMLAttributes<HTMLModElement>
  details: DetailsHTMLAttributes<HTMLDetailsElement>
  dfn: HTMLAttributes<HTMLElement>
  dialog: DialogHTMLAttributes<HTMLDialogElement>
  div: HTMLAttributes<HTMLDivElement>
  dl: HTMLAttributes<HTMLDListElement>
  dt: HTMLAttributes<HTMLElement>
  em: HTMLAttributes<HTMLElement>
  embed: EmbedHTMLAttributes<HTMLEmbedElement>
  fieldset: FieldsetHTMLAttributes<HTMLFieldSetElement>
  figcaption: HTMLAttributes<HTMLElement>
  figure: HTMLAttributes<HTMLElement>
  footer: HTMLAttributes<HTMLElement>
  form: FormHTMLAttributes<HTMLFormElement>
  h1: HTMLAttributes<HTMLHeadingElement>
  h2: HTMLAttributes<HTMLHeadingElement>
  h3: HTMLAttributes<HTMLHeadingElement>
  h4: HTMLAttributes<HTMLHeadingElement>
  h5: HTMLAttributes<HTMLHeadingElement>
  h6: HTMLAttributes<HTMLHeadingElement>
  head: HTMLAttributes<HTMLHeadElement>
  header: HTMLAttributes<HTMLElement>
  hgroup: HTMLAttributes<HTMLElement>
  hr: HTMLAttributes<HTMLHRElement>
  html: HtmlHTMLAttributes<HTMLHtmlElement>
  i: HTMLAttributes<HTMLElement>
  iframe: IframeHTMLAttributes<HTMLIFrameElement>
  img: ImgHTMLAttributes<HTMLImageElement>
  input: InputHTMLAttributes<HTMLInputElement>
  ins: InsHTMLAttributes<HTMLModElement>
  kbd: HTMLAttributes<HTMLElement>
  keygen: KeygenHTMLAttributes<HTMLElement>
  label: LabelHTMLAttributes<HTMLLabelElement>
  legend: HTMLAttributes<HTMLLegendElement>
  li: LiHTMLAttributes<HTMLLIElement>
  link: LinkHTMLAttributes<HTMLLinkElement>
  main: HTMLAttributes<HTMLElement>
  map: MapHTMLAttributes<HTMLMapElement>
  mark: HTMLAttributes<HTMLElement>
  menu: MenuHTMLAttributes<HTMLElement>
  menuitem: HTMLAttributes<HTMLElement>
  meta: MetaHTMLAttributes<HTMLMetaElement>
  meter: MeterHTMLAttributes<HTMLMeterElement>
  nav: HTMLAttributes<HTMLElement>
  noindex: HTMLAttributes<HTMLElement>
  noscript: HTMLAttributes<HTMLObjectElement>
  object: ObjectHTMLAttributes<HTMLObjectElement>
  ol: OlHTMLAttributes<HTMLOListElement>
  optgroup: OptgroupHTMLAttributes<HTMLOptGroupElement>
  option: OptionHTMLAttributes<HTMLOptionElement>
  output: OutputHTMLAttributes<HTMLOutputElement>
  p: HTMLAttributes<HTMLParagraphElement>
  param: ParamHTMLAttributes<HTMLParamElement>
  picture: HTMLAttributes<HTMLElement>
  pre: HTMLAttributes<HTMLPreElement>
  progress: ProgressHTMLAttributes<HTMLProgressElement>
  q: QuoteHTMLAttributes<HTMLQuoteElement>
  rp: HTMLAttributes<HTMLElement>
  rt: HTMLAttributes<HTMLElement>
  ruby: HTMLAttributes<HTMLElement>
  s: HTMLAttributes<HTMLElement>
  samp: HTMLAttributes<HTMLElement>
  search: HTMLAttributes<HTMLElement>
  script: ScriptHTMLAttributes<HTMLScriptElement>
  section: HTMLAttributes<HTMLElement>
  select: SelectHTMLAttributes<HTMLSelectElement>
  small: HTMLAttributes<HTMLElement>
  source: SourceHTMLAttributes<HTMLSourceElement>
  span: HTMLAttributes<HTMLSpanElement>
  strong: HTMLAttributes<HTMLElement>
  style: StyleHTMLAttributes<HTMLStyleElement>
  sub: HTMLAttributes<HTMLElement>
  summary: HTMLAttributes<HTMLElement>
  sup: HTMLAttributes<HTMLElement>
  table: TableHTMLAttributes<HTMLTableElement>
  template: HTMLAttributes<HTMLTemplateElement>
  tbody: HTMLAttributes<HTMLTableSectionElement>
  td: TdHTMLAttributes<HTMLTableDataCellElement>
  textarea: TextareaHTMLAttributes<HTMLTextAreaElement>
  tfoot: HTMLAttributes<HTMLTableSectionElement>
  th: ThHTMLAttributes<HTMLTableHeaderCellElement>
  thead: HTMLAttributes<HTMLTableSectionElement>
  time: TimeHTMLAttributes<HTMLTimeElement>
  title: HTMLAttributes<HTMLTitleElement>
  tr: HTMLAttributes<HTMLTableRowElement>
  track: TrackHTMLAttributes<HTMLTrackElement>
  u: HTMLAttributes<HTMLElement>
  ul: HTMLAttributes<HTMLUListElement>
  var: HTMLAttributes<HTMLElement>
  video: VideoHTMLAttributes<HTMLVideoElement>
  wbr: HTMLAttributes<HTMLElement>
  webview: WebViewHTMLAttributes<HTMLWebViewElement>

  // SVG
  svg: SVGAttributes

  animate: SVGAttributes
  animateMotion: SVGAttributes
  animateTransform: SVGAttributes
  circle: SVGAttributes
  clipPath: SVGAttributes
  defs: SVGAttributes
  desc: SVGAttributes
  ellipse: SVGAttributes
  feBlend: SVGAttributes
  feColorMatrix: SVGAttributes
  feComponentTransfer: SVGAttributes
  feComposite: SVGAttributes
  feConvolveMatrix: SVGAttributes
  feDiffuseLighting: SVGAttributes
  feDisplacementMap: SVGAttributes
  feDistantLight: SVGAttributes
  feDropShadow: SVGAttributes
  feFlood: SVGAttributes
  feFuncA: SVGAttributes
  feFuncB: SVGAttributes
  feFuncG: SVGAttributes
  feFuncR: SVGAttributes
  feGaussianBlur: SVGAttributes
  feImage: SVGAttributes
  feMerge: SVGAttributes
  feMergeNode: SVGAttributes
  feMorphology: SVGAttributes
  feOffset: SVGAttributes
  fePointLight: SVGAttributes
  feSpecularLighting: SVGAttributes
  feSpotLight: SVGAttributes
  feTile: SVGAttributes
  feTurbulence: SVGAttributes
  filter: SVGAttributes
  foreignObject: SVGAttributes
  g: SVGAttributes
  image: SVGAttributes
  line: SVGAttributes
  linearGradient: SVGAttributes
  marker: SVGAttributes
  mask: SVGAttributes
  metadata: SVGAttributes
  mpath: SVGAttributes
  path: SVGAttributes
  pattern: SVGAttributes
  polygon: SVGAttributes
  polyline: SVGAttributes
  radialGradient: SVGAttributes
  rect: SVGAttributes
  stop: SVGAttributes
  switch: SVGAttributes
  symbol: SVGAttributes
  text: SVGAttributes
  textPath: SVGAttributes
  tspan: SVGAttributes
  use: SVGAttributes
  view: SVGAttributes
}

export interface Events<T = Element> {
  // clipboard events
  onCopy: ClipboardEventHandler<T>
  onCut: ClipboardEventHandler<T>
  onPaste: ClipboardEventHandler<T>

  // composition events
  onCompositionend: CompositionEventHandler<T>
  onCompositionstart: CompositionEventHandler<T>
  onCompositionupdate: CompositionEventHandler<T>

  // drag drop events
  onDrag: DragEventHandler<T>
  onDragend: DragEventHandler<T>
  onDragenter: DragEventHandler<T>
  onDragexit: DragEventHandler<T>
  onDragleave: DragEventHandler<T>
  onDragover: DragEventHandler<T>
  onDragstart: DragEventHandler<T>
  onDrop: DragEventHandler<T>

  // focus events
  onFocus: FocusEventHandler<T>
  onFocusin: FocusEventHandler<T>
  onFocusout: FocusEventHandler<T>
  onBlur: FocusEventHandler<T>

  // form events
  onChange: ChangeEventHandler<T>
  onBeforeinput: FormEventHandler<T>
  onInput: FormEventHandler<T>
  onReset: FormEventHandler<T>
  onSubmit: FormEventHandler<T>
  onInvalid: FormEventHandler<T>

  // image events
  onLoad: BaseEventHandler<T>
  onError: BaseEventHandler<T>

  // keyboard events
  onKeydown: KeyboardEventHandler<T>
  onKeypress: KeyboardEventHandler<T>
  onKeyup: KeyboardEventHandler

  // mouse events
  onAuxclick: MouseEventHandler<T>
  onClick: MouseEventHandler<T>
  onContextmenu: MouseEventHandler<T>
  onDblclick: MouseEventHandler<T>
  onMousedown: MouseEventHandler<T>
  onMouseenter: MouseEventHandler<T>
  onMouseleave: MouseEventHandler<T>
  onMousemove: MouseEventHandler<T>
  onMouseout: MouseEventHandler<T>
  onMouseover: MouseEventHandler<T>
  onMouseup: MouseEventHandler<T>

  // media events
  onAbort: BaseEventHandler<T>
  onCanplay: BaseEventHandler<T>
  onCanplaythrough: BaseEventHandler<T>
  onDurationchange: BaseEventHandler<T>
  onEmptied: BaseEventHandler<T>
  onEncrypted: BaseEventHandler<T>
  onEnded: BaseEventHandler<T>
  onLoadeddata: BaseEventHandler<T>
  onLoadedmetadata: BaseEventHandler<T>
  onLoadstart: BaseEventHandler<T>
  onPause: BaseEventHandler<T>
  onPlay: BaseEventHandler<T>
  onPlaying: BaseEventHandler<T>
  onProgress: BaseEventHandler<T>
  onRatechange: BaseEventHandler<T>
  onSeeked: BaseEventHandler<T>
  onSeeking: BaseEventHandler<T>
  onStalled: BaseEventHandler<T>
  onSuspend: BaseEventHandler<T>
  onTimeupdate: BaseEventHandler<T>
  onVolumechange: BaseEventHandler<T>
  onWaiting: BaseEventHandler<T>

  // selection events
  onSelect: BaseEventHandler<T>

  // scroll events
  onScroll: UIEventHandler<T>
  onScrollend: UIEventHandler<T>

  // touch events
  onTouchcancel: TouchEvent
  onTouchend: TouchEvent
  onTouchmove: TouchEvent
  onTouchstart: TouchEvent

  // pointer events
  onPointerdown: PointerEvent
  onPointermove: PointerEvent
  onPointerup: PointerEvent
  onPointercancel: PointerEvent
  onPointerenter: PointerEvent
  onPointerleave: PointerEvent
  onPointerover: PointerEvent
  onPointerout: PointerEvent

  // wheel events
  onWheel: WheelEventHandler<T>

  // animation events
  onAnimationstart: AnimationEventHandler<T>
  onAnimationend: AnimationEventHandler<T>
  onAnimationiteration: AnimationEventHandler<T>

  // transition events
  onTransitionend: TransitionEventHandler<T>
  onTransitionstart: TransitionEventHandler<T>
}

export type EventHandlers<E> = {
  [K in keyof E]?: E[K] extends (...args: any) => any
    ? E[K]
    : (payload: E[K]) => void
}

type _ReservedProps = import('vue').ReservedProps
export interface ReservedProps extends _ReservedProps {}

export type NativeElements = {
  [K in keyof IntrinsicElementAttributes]: IntrinsicElementAttributes[K] &
    ReservedProps
}

export interface BaseSyntheticEvent<E = object, C = unknown, T = unknown> {
  nativeEvent: E
  currentTarget: C
  target: T
  bubbles: boolean
  cancelable: boolean
  defaultPrevented: boolean
  eventPhase: number
  isTrusted: boolean
  preventDefault: () => void
  isDefaultPrevented: () => boolean
  stopPropagation: () => void
  isPropagationStopped: () => boolean
  persist: () => void
  timeStamp: number
  type: string
}

/**
 * currentTarget - a reference to the element on which the event listener is registered.
 *
 * target - a reference to the element from which the event was originally dispatched.
 * This might be a child element to the element on which the event listener is registered.
 * If you thought this should be `EventTarget & T`, see https://github.com/DefinitelyTyped/DefinitelyTyped/issues/11508#issuecomment-256045682
 */
export interface SyntheticEvent<T = Element, E = Event>
  extends BaseSyntheticEvent<E, EventTarget & T, EventTarget> {}

export type EventHandler<E extends SyntheticEvent<any>> = {
  bivarianceHack: (event: E) => void
}['bivarianceHack']

export type BaseEventHandler<T = Element> = EventHandler<SyntheticEvent<T>>

export interface ClipboardEvent<T = Element>
  extends SyntheticEvent<T, globalThis.ClipboardEvent> {
  clipboardData: DataTransfer
}
export type ClipboardEventHandler<T = Element> = EventHandler<ClipboardEvent<T>>

export interface CompositionEvent<T = Element>
  extends SyntheticEvent<T, globalThis.CompositionEvent> {
  data: string
}
export type CompositionEventHandler<T = Element> = EventHandler<
  CompositionEvent<T>
>

export interface DragEvent<T = Element>
  extends MouseEvent<T, globalThis.DragEvent> {
  dataTransfer: DataTransfer
}
export type DragEventHandler<T = Element> = EventHandler<DragEvent<T>>

export interface FocusEvent<Target = Element, RelatedTarget = Element>
  extends SyntheticEvent<Target, globalThis.FocusEvent> {
  relatedTarget: (EventTarget & RelatedTarget) | null
  target: EventTarget & Target
}
export type FocusEventHandler<T = Element> = EventHandler<FocusEvent<T>>

export interface FormEvent<T = Element> extends SyntheticEvent<T> {}
export type FormEventHandler<T = Element> = EventHandler<FormEvent<T>>

export interface ChangeEvent<T = Element> extends SyntheticEvent<T> {
  target: EventTarget & T
}
export type ChangeEventHandler<T = Element> = EventHandler<ChangeEvent<T>>

export interface KeyboardEvent<T = Element>
  extends UIEvent<T, globalThis.KeyboardEvent> {
  altKey: boolean
  /** @deprecated */
  charCode: number
  ctrlKey: boolean
  code: string
  /**
   * See [DOM Level 3 Events spec](https://www.w3.org/TR/uievents-key/#keys-modifier). for a list of valid (case-sensitive) arguments to this method.
   */
  getModifierState: (key: ModifierKey) => boolean
  /**
   * See the [DOM Level 3 Events spec](https://www.w3.org/TR/uievents-key/#named-key-attribute-values). for possible values
   */
  key: string
  /** @deprecated */
  keyCode: number
  locale: string
  location: number
  metaKey: boolean
  repeat: boolean
  shiftKey: boolean
  /** @deprecated */
  which: number
}
export type KeyboardEventHandler<T = Element> = EventHandler<KeyboardEvent<T>>

export type ModifierKey =
  | 'Alt'
  | 'AltGraph'
  | 'CapsLock'
  | 'Control'
  | 'Fn'
  | 'FnLock'
  | 'Hyper'
  | 'Meta'
  | 'NumLock'
  | 'ScrollLock'
  | 'Shift'
  | 'Super'
  | 'Symbol'
  | 'SymbolLock'
export interface MouseEvent<T = Element, E = globalThis.MouseEvent>
  extends UIEvent<T, E> {
  altKey: boolean
  button: number
  buttons: number
  clientX: number
  clientY: number
  ctrlKey: boolean
  /**
   * See [DOM Level 3 Events spec](https://www.w3.org/TR/uievents-key/#keys-modifier). for a list of valid (case-sensitive) arguments to this method.
   */
  getModifierState: (key: ModifierKey) => boolean
  metaKey: boolean
  movementX: number
  movementY: number
  pageX: number
  pageY: number
  relatedTarget: EventTarget | null
  screenX: number
  screenY: number
  shiftKey: boolean
}
export type MouseEventHandler<T = Element> = EventHandler<MouseEvent<T>>

export interface AbstractView {
  styleMedia: StyleMedia
  document: Document
}
export interface UIEvent<T = Element, E = globalThis.UIEvent>
  extends SyntheticEvent<T, E> {
  detail: number
  view: AbstractView
}
export type UIEventHandler<T = Element> = EventHandler<UIEvent<T>>

export interface WheelEvent<T = Element>
  extends MouseEvent<T, globalThis.WheelEvent> {
  deltaMode: number
  deltaX: number
  deltaY: number
  deltaZ: number
}
export type WheelEventHandler<T = Element> = EventHandler<WheelEvent<T>>

export interface AnimationEvent<T = Element>
  extends SyntheticEvent<T, globalThis.AnimationEvent> {
  animationName: string
  elapsedTime: number
  pseudoElement: string
}
export type AnimationEventHandler<T = Element> = EventHandler<AnimationEvent<T>>

export interface TransitionEvent<T = Element>
  extends SyntheticEvent<T, globalThis.TransitionEvent> {
  elapsedTime: number
  propertyName: string
  pseudoElement: string
}
export type TransitionEventHandler<T = Element> = EventHandler<
  TransitionEvent<T>
>
