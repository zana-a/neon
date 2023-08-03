import type { Meta, StoryObj } from "@storybook/react";
import { Container, Navbar } from "@/components";

export const Default: StoryObj<typeof Navbar> = {
  render: () => <Navbar />,
};

export const Collection: StoryObj<typeof Navbar> = {
  render: () => (
    <Navbar>
      <Container>
        <Navbar.Collection>
          <Navbar.Link href="#">Learn</Navbar.Link>
          <Navbar.Link href="#">Documentation</Navbar.Link>
          <Navbar.Link href="#">Blog</Navbar.Link>
        </Navbar.Collection>
      </Container>
    </Navbar>
  ),
};

const meta: Meta<typeof Navbar> = {
  component: Navbar,
};

export default meta;
