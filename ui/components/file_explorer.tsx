import { LinearProgress, Stack } from '@mui/joy'
import { NavigationBreadcrumbs } from './breadcrumbs'
import { FileSystemList } from './file_system_list'
import { use_tree_data } from '../hooks/use_tree_data'
import { use_tree_navigation } from '../hooks/use_tree_navigation'
import { P } from '../shortcuts'

export const FileExplorer = () => {
	const { tree, is_loading, error } = use_tree_data()
	const { path_stack, current_nodes, navigate_to_dir, navigate_to_breadcrumb } =
		use_tree_navigation(tree)

	if (is_loading) {
		return <LinearProgress color="neutral" size="sm" />
	}

	if (error) {
		return <P color="danger">Error: {error}</P>
	}

	const current_path =
		path_stack.length > 0
			? '/' + path_stack.map(node => node.name).join('/')
			: ''

	return (
		<Stack gap={2}>
			<NavigationBreadcrumbs
				path_stack={path_stack}
				on_navigate={navigate_to_breadcrumb}
			/>
			<FileSystemList
				nodes={current_nodes}
				on_node_click={navigate_to_dir}
				current_path={current_path}
			/>
		</Stack>
	)
}
