import type { PageServerLoad } from "./$types";
import { networkInterfaces } from "os";

export const ssr = false;
export const prerender = false;

export const load = (({}) => {
  var en0 = networkInterfaces()["en0"];
  let serverNetworkInterfaceInfo =
    en0?.filter((item) => item.family === "IPv4")[0];

  return {
    serverNetworkInterfaceInfo,
  };
}) satisfies PageServerLoad;
