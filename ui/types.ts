export type Node =
	| { type: 'File'; name: string; id: string }
	| { type: 'Dir'; name: string; children: Node[] }

export interface Resp<T> {
	data: T
	error: string
}

export type Tree = Node[]
