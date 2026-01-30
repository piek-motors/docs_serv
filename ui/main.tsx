import { Container, Stack } from '@mui/joy'
import { render } from 'preact'
import { FileExplorer } from './components/file_explorer'
import logoSvg from './assets/logo.svg'
import { P } from './shortcuts'
import Router from 'preact-router'

const Header = () => (
	<Stack
		direction={'row'}
		gap={2}
		alignItems={'center'}
		sx={{ borderBottom: 1, borderColor: 'lightgray' }}
		justifyContent={'center'}
	>
		<a href={'https://piek.ru'}>
			<img src={logoSvg} width={60} />
		</a>
		<a href={'/'} style={{ textDecoration: 'none' }}>
			<P mb={0.5}>Документация</P>
		</a>
	</Stack>
)

const App = () => {
	return (
		<Container maxWidth="md">
			<Header />
			<Router>
				<FileExplorer path="/:path*" />
			</Router>
		</Container>
	)
}

render(<App />, document.getElementById('app')!)
