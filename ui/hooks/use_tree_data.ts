import { useEffect, useState } from 'preact/hooks'
import type { Tree } from '../types'
import { sort_tree } from '../utils/tree_utils'

export const with_base_url = (path: string) => {
	return import.meta.env.VITE_BASE_URL + path
}

export const use_tree_data = () => {
	const [tree, set_tree] = useState<Tree>()
	const [is_loading, set_is_loading] = useState(true)
	const [error, set_error] = useState<string | null>(null)

	useEffect(() => {
		fetch(with_base_url('/api/ls'), { method: 'GET' })
			.then(res => res.json())
			.then(resp => {
				const tree = sort_tree(resp.data as Tree)
				set_tree(tree)
				set_is_loading(false)
			})
			.catch(err => {
				set_error(err.message)
				set_is_loading(false)
			})
	}, [])

	return { tree, is_loading, error }
}
