// hooks/use_tree_navigation.ts
import { useState, useEffect } from 'preact/hooks'
import type { Tree, FsNode } from '../types'
import { sort_tree } from '../utils/tree_utils'

const find_node_by_path = (
	tree: Tree,
	path_parts: string[],
): { node: FsNode | null; stack: FsNode[] } => {
	if (path_parts.length === 0) {
		return { node: null, stack: [] }
	}

	let current_tree: FsNode[] = tree
	const stack: FsNode[] = []

	for (const part of path_parts) {
		const found = current_tree.find(
			node => node.name === part && node.type === 'Dir',
		)
		if (!found || found.type !== 'Dir') {
			return { node: null, stack: [] }
		}
		stack.push(found)
		current_tree = found.children
	}

	return { node: stack[stack.length - 1], stack }
}

export const use_tree_navigation = (
	tree: Tree | undefined,
	url_path_parts: string[],
) => {
	const [path_stack, set_path_stack] = useState<FsNode[]>([])
	const [current_nodes, set_current_nodes] = useState<FsNode[]>([])

	useEffect(() => {
		if (!tree) return

		if (url_path_parts.length === 0) {
			set_path_stack([])
			set_current_nodes(tree)
		} else {
			const { node, stack } = find_node_by_path(tree, url_path_parts)

			if (node && node.type === 'Dir') {
				set_path_stack(stack)
				set_current_nodes(node.children)
			} else {
				console.log('Invalid path, setting root')
				set_path_stack([])
				set_current_nodes(tree)
			}
		}
	}, [tree, url_path_parts.join('/')])

	const navigate_to_dir = (node: FsNode) => {
		if (node.type === 'Dir') {
			const new_stack = [...path_stack, node]
			set_path_stack(new_stack)
			set_current_nodes(node.children)
		}
	}

	const navigate_to_breadcrumb = (index: number) => {
		if (!tree) return

		if (index === -1) {
			set_path_stack([])
			set_current_nodes(tree)
		} else {
			const new_path = path_stack.slice(0, index + 1)
			set_path_stack(new_path)
			const target_node = new_path[new_path.length - 1]
			if (target_node && target_node.type === 'Dir') {
				set_current_nodes(target_node.children)
			}
		}
	}

	return {
		path_stack,
		current_nodes: sort_tree(current_nodes),
		navigate_to_dir,
		navigate_to_breadcrumb,
	}
}
