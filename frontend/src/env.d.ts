interface ProcessEnv {
  REVERSI_VERSION?: string;
}

declare const process: {
  env: ProcessEnv;
};
