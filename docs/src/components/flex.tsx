import React from "react";

enum FlexMode {
  Default,
  Auto,
  None,
}

enum JustifyContent {
  Default,
  End,
}

interface FlexProps {
  children: React.ReactNode;
  justifyContent: JustifyContent;
  flexMode: FlexMode;
}

const getJustifyContent = (justifyContent: JustifyContent) => {
  if (justifyContent === JustifyContent.End) return "justify-end";
  return "";
};

const getFlexMode = (flexMode: FlexMode) => {
  if (flexMode === FlexMode.Auto) return "flex-auto";
  return "";
};

const Flex = (props: FlexProps) => {
  const justifyContent = getJustifyContent(props.justifyContent);
  const flexMode = getFlexMode(props.flexMode);
  return (
    <div className={`flex ${flexMode} ${justifyContent}`}>{props.children}</div>
  );
};

Flex.defaultProps = {
  justifyContent: JustifyContent.Default,
  flexMode: JustifyContent.Default,
};

export default Flex;
export { JustifyContent, FlexMode };
