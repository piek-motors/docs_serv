import { useState, useEffect } from 'preact/hooks'
import type { Tree, Node } from '../types'
import { sort_tree } from '../utils/tree_utils'

export const use_tree_navigation = (tree: Tree | undefined) => {
	const [path_stack, set_path_stack] = useState<Node[]>([])
	const [current_nodes, set_current_nodes] = useState<Node[]>([])

	useEffect(() => {
		if (!tree) return
		set_current_nodes(tree)
	}, [tree])

	const navigate_to_dir = (node: Node) => {
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
			if (target_node.type === 'Dir') {
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
