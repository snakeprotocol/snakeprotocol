import React from "react";
import Admonition from "@theme/Admonition";

const SupportedEnvironments = () => {
  return (
    <Admonition type="info" title="Supported Environments">
      The Goose CLI currently works on <strong>macOS</strong> and <strong>Linux</strong> systems and supports both <strong>ARM</strong> and <strong>x86</strong> architectures. However, the macOS desktop app is not currently supported for x86. 

      On <strong>Windows</strong>, Goose CLI can run via WSL. If you'd like to request support for additional operating systems, please{" "}
      <a
        href="https://github.com/block/goose/discussions/867"
        target="_blank"
        rel="noopener noreferrer"
      >
        vote on GitHub
      </a>.
    </Admonition>
  );
};

export default SupportedEnvironments;
