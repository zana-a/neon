import type { Meta, StoryObj } from "@storybook/react";
import { Container } from "@/components";

export const Default: StoryObj<typeof Container> = {
  render: () => (
    <Container>
      <p>This is an example container.</p>
      <p>Nothing special... move along.</p>
    </Container>
  ),
};

const meta: Meta<typeof Container> = {
  component: Container,
};

export default meta;
