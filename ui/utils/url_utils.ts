import type { Node } from '../types'

export const update_browser_url = (stack: Node[]) => {
	const path = '/' + stack.map(node => node.name).join('/')
	window.history.pushState({}, '', path)
}

export const get_current_path_parts = (): string[] => {
	return window.location.pathname.split('/').filter(p => p !== '')
}
