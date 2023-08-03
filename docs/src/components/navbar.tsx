import Link from "next/link";
import React from "react";

interface NavbarProps {
  children?: React.ReactNode;
}

const Navbar = (props: NavbarProps) => {
  return (
    <div className="fixed top-0 left-0 w-full">
      <nav className="border-b border-gray-200 bg-white h-14 flex items-center">
        {props.children}
      </nav>
    </div>
  );
};

interface NavbarCollectionProps {
  children: React.ReactNode | React.ReactNode[];
}

const NavbarCollection = (props: NavbarCollectionProps) => {
  let items;
  if (props.children instanceof Array) {
    items = props.children.map((item) => <li key={1}>{item}</li>);
  } else {
    items = <li>{props.children}</li>;
  }
  return <ul className="list-none flex space-x-3 [&>*]:flex">{items}</ul>;
};
Navbar.Collection = NavbarCollection;

interface NavbarLinkProps {
  children: React.ReactNode;
  href: string;
}

const NavbarLink = (props: NavbarLinkProps) => {
  return (
    <Link
      href={props.href}
      className="hover:text-pink-500 transition-all inline-flex text-sm my-auto"
    >
      {props.children}
    </Link>
  );
};
Navbar.Link = NavbarLink;

export default Navbar;
