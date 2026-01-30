import { Breadcrumbs, Link } from '@mui/joy'
import type { FsNode } from '../types'

interface NavigationBreadcrumbsProps {
	path_stack: FsNode[]
	on_navigate: (index: number) => void
}

export const NavigationBreadcrumbs = ({
	path_stack,
	on_navigate,
}: NavigationBreadcrumbsProps) => {
	return (
		<Breadcrumbs
			size="sm"
			slotProps={{ separator: { sx: { fontSize: '.8rem' } } }}
		>
			<Link
				level="body-sm"
				fontWeight={'normal'}
				component="button"
				onClick={() => on_navigate(-1)}
				color={'neutral'}
			>
				Главная
			</Link>
			{path_stack.map((node, index) => (
				<Link
					level="body-sm"
					fontWeight={'normal'}
					key={index}
					component="button"
					onClick={() => on_navigate(index)}
					color={'neutral'}
				>
					{node.name}
				</Link>
			))}
		</Breadcrumbs>
	)
}
