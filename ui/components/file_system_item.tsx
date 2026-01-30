import { IconButton, Link, Stack } from '@mui/joy'
import { P } from '../shortcuts'
import type { Node } from '../types'
import { with_base_url } from '../hooks/use_tree_data'
import LinkIcon from '../assets/link_icon.svg'
import FileIcon from '../assets/file_icon.svg'
import FolderIcon from '../assets/folder_icon.svg'

interface FileSystemItemProps {
	node: Node
	on_click: (node: Node) => void
	current_path: string
}

export const FileSystemItem = ({
	node,
	on_click,
	current_path,
}: FileSystemItemProps) => {
	const is_directory = node.type === 'Dir'
	const icon = is_directory ? FolderIcon : FileIcon

	const file_url = (() => {
		if (node.type === 'Dir') return null
		if (node.id) return `/file/${node.id}`
		return `/browse${current_path}/${node.name}`
	})()

	const handle_copy_link = async (e: MouseEvent) => {
		e.stopPropagation()
		e.preventDefault()

		if (!file_url) return
		try {
			const full_url = window.location.origin + file_url
			await navigator.clipboard.writeText(full_url)
		} catch (err) {
			console.error('Failed to copy link:', err)
		}
	}

	const handle_click = (e: MouseEvent) => {
		if (is_directory) {
			e.preventDefault()
			on_click(node)
		}
	}

	const has_file_id = node.type === 'File' && node.id
	const name = has_file_id ? node.name.replace(node.id, '').slice(1) : node.name

	return (
		<Stack direction="row" alignItems="center">
			<Link
				href={with_base_url(file_url ?? '') || '#'}
				target={is_directory ? undefined : '_blank'}
				rel={is_directory ? undefined : 'noopener noreferrer'}
				onClick={handle_click}
				style={{
					flexGrow: 1,
					cursor: 'pointer',
					textDecorationColor: 'grey',
					color: 'inherit',
				}}
				sx={{ display: 'flex', gap: 1, alignItems: 'center' }}
			>
				<img src={icon} width={16} />
				<P>{name}</P>
				{has_file_id && (
					<P color="neutral" level="body-xs">
						({node.id})
					</P>
				)}
			</Link>

			{!is_directory && (
				<IconButton
					size="sm"
					variant="plain"
					onClick={handle_copy_link}
					title="Copy link"
				>
					<img src={LinkIcon} width={16} />
				</IconButton>
			)}
		</Stack>
	)
}
