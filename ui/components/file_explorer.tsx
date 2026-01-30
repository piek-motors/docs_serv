// components/file_explorer.tsx
import { LinearProgress, Stack } from '@mui/joy'
import { NavigationBreadcrumbs } from './breadcrumbs'
import { FileSystemList } from './file_system_list'
import { use_tree_data } from '../hooks/use_tree_data'
import { use_tree_navigation } from '../hooks/use_tree_navigation'
import { P } from '../shortcuts'
import { route } from 'preact-router'
import type { FsNode } from '../types'

interface FileExplorerProps {
	path?: string
}

export const FileExplorer = ({ path }: FileExplorerProps) => {
	const { tree, is_loading, error } = use_tree_data()
	// Parse path from URL (path will be like "folder1/folder2" or undefined for root)
	const url_path_parts = path ? path.split('/').filter(p => p) : []

	const { path_stack, current_nodes, navigate_to_dir, navigate_to_breadcrumb } =
		use_tree_navigation(tree, url_path_parts)

	if (is_loading) {
		return <LinearProgress color="neutral" size="sm" />
	}
	if (error) {
		return <P color="danger">Error: {error}</P>
	}

	// Build current path from stack
	const current_path =
		path_stack.length > 0
			? '/' + path_stack.map(node => node.name).join('/')
			: ''

	// Wrapper functions to update URL
	const handle_navigate_to_dir = (node: FsNode) => {
		if (node.type === 'Dir') {
			navigate_to_dir(node)
			const new_path = [...path_stack, node].map(n => n.name).join('/')
			route('/' + new_path)
		}
	}

	const handle_navigate_to_breadcrumb = (index: number) => {
		navigate_to_breadcrumb(index)
		if (index === -1) {
			route('/')
		} else {
			const new_path = path_stack
				.slice(0, index + 1)
				.map(n => n.name)
				.join('/')
			route('/' + new_path)
		}
	}

	return (
		<Stack gap={2}>
			<NavigationBreadcrumbs
				path_stack={path_stack}
				on_navigate={handle_navigate_to_breadcrumb}
			/>
			<FileSystemList
				nodes={current_nodes}
				on_node_click={handle_navigate_to_dir}
				current_path={current_path}
			/>
		</Stack>
	)
}
