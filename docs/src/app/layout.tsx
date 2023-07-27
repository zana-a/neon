import "./globals.css";

interface RootLayoutProps {
  children: React.ReactNode;
}

const RootLayout = (props: RootLayoutProps) => {
  return <>{props.children}</>;
};

export default RootLayout;
