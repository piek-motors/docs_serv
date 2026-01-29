import { Container, Stack } from '@mui/joy'
import { render } from 'preact'
import { FileExplorer } from './components/file_explorer'
import logoSvg from './assets/logo.svg'
import { P } from './shortcuts'

const Header = () => (
    <Stack direction={'row'} gap={2} alignItems={'center'} sx={{ borderBottom: 1, borderColor: 'lightgray' }} justifyContent={'center'}>
        <a href={'https://piek.ru'}>
            <img src={logoSvg} width={60} />
        </a>
        <P mb={.5}>Документация</P>
    </Stack>
)

const App = () => {
    return (
        <Container maxWidth='sm'>
            <Header />
            <FileExplorer />
        </Container>
    )
}

render(<App />, document.getElementById('app')!)
