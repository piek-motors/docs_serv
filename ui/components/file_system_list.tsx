import { Stack } from '@mui/joy'
import type { Node } from '../types'
import { FileSystemItem } from './file_system_item'

interface FileSystemListProps {
	nodes: Node[]
	on_node_click: (node: Node) => void
	current_path: string
}

export const FileSystemList = ({
	nodes,
	on_node_click,
	current_path,
}: FileSystemListProps) => {
	return (
		<Stack gap={1}>
			{nodes.map((node, idx) => (
				<FileSystemItem
					key={idx}
					node={node}
					on_click={on_node_click}
					current_path={current_path}
				/>
			))}
		</Stack>
	)
}
