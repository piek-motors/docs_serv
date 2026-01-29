import type { Tree } from '../types';

export const sort_tree = (tree: Tree): Tree => {
	return tree.toSorted((a, b) => {
		if (a.type !== b.type) {
			return a.type === 'Dir' ? -1 : 1
		}

		return a.name.localeCompare(b.name, 'ru', {
			numeric: true,
			sensitivity: 'base',
		})
	})
}