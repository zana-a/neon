import React from "react";

enum DisplayMode {
  Flex,
  Block,
}

interface ContainerProps {
  children: React.ReactNode;
  displayMode: DisplayMode;
}

const getDisplayMode = (displayMode: DisplayMode) => {
  if (displayMode === DisplayMode.Flex) return "flex";
  return "block";
};

const Container = (props: ContainerProps) => {
  const displayMode = getDisplayMode(props.displayMode);
  return (
    <div className={`container mx-auto px-4 ${displayMode}`}>
      {props.children}
    </div>
  );
};

Container.defaultProps = {
  displayMode: DisplayMode.Block,
};

export default Container;
export { DisplayMode };
