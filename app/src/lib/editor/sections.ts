export const editorSections = [
  { id: 'scenario', label: 'Scenario' },
  { id: 'status', label: 'Status' },
] as const;

export type EditorSectionId = (typeof editorSections)[number]['id'];
