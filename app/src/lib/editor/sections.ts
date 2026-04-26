export const editorSections = [
  { id: 'header', label: 'Header' },
  { id: 'diagnostics', label: 'Diagnostics' },
] as const;

export type EditorSectionId = (typeof editorSections)[number]['id'];
