import "./globals.css";
import GithubLogo from "@/app/github.svg";

import {
  Container,
  DisplayMode,
  Flex,
  FlexMode,
  JustifyContent,
  Navbar,
} from "@/components";

interface RootLayoutProps {
  children: React.ReactNode;
}

const RootLayout = (props: RootLayoutProps) => {
  return (
    <html lang="en">
      <head>
        <meta charSet="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Document</title>
      </head>
      <body>
        <Navbar>
          <Container displayMode={DisplayMode.Flex}>
            <Flex flexMode={FlexMode.Auto}>
              <span>Logo</span>
            </Flex>
            <Flex justifyContent={JustifyContent.End}>
              <div className="space-x-4 flex">
                <Navbar.Collection>
                  <Navbar.Link href="/">Home</Navbar.Link>
                  <Navbar.Link href="/learn">Learn</Navbar.Link>
                  <Navbar.Link href="/docs">Documentation</Navbar.Link>
                  <Navbar.Link href="/blog">Blog</Navbar.Link>
                </Navbar.Collection>
                <hr className="my-auto w-1 h-6 border-0 bg-gray-100" />
                <Navbar.Collection>
                  <Navbar.Link href="https://github.com/zana-a/neon/">
                    <GithubLogo
                      className={"w-14 hover:fill-pink-500 transition-all"}
                    />
                  </Navbar.Link>
                </Navbar.Collection>
              </div>
            </Flex>
          </Container>
        </Navbar>
        <main className="mt-14">{props.children}</main>
      </body>
    </html>
  );
};

export default RootLayout;
