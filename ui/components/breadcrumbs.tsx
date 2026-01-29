import { Breadcrumbs, Link } from '@mui/joy'
import type { Node } from '../types'

interface NavigationBreadcrumbsProps {
    path_stack: Node[]
    on_navigate: (index: number) => void
}

export const NavigationBreadcrumbs = ({
    path_stack,
    on_navigate,
}: NavigationBreadcrumbsProps) => {
    return (
        <Breadcrumbs size='sm' slotProps={{ separator: { sx: { fontSize: '.6rem' } } }}>
            <Link
                level='body-xs'
                fontWeight={'normal'}
                component="button"
                onClick={() => on_navigate(-1)}
                color={'neutral'}
            >
                Главная
            </Link>
            {path_stack.map((node, index) => (
                <Link
                    level='body-xs'
                    fontWeight={'normal'}
                    key={index}
                    component="button"
                    onClick={() => on_navigate(index)}
                    color={index === path_stack.length - 1 ? 'neutral' : 'primary'}
                >
                    {node.name}
                </Link>
            ))}
        </Breadcrumbs>
    )
}
