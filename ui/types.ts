export type FsNode =
	| { type: 'File'; name: string; id: string }
	| { type: 'Dir'; name: string; children: FsNode[] }

export interface Resp<T> {
	data: T
	error: string
}

export type Tree = FsNode[]
