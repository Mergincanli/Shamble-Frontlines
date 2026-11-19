// The render folder's one entry (picture.ts, `CreatePicture`): what the app shell imports, and nothing else here.
import type { CreatePicture } from "../picture.ts";
import { createThreePicture } from "./three-picture.ts";

export const createPicture: CreatePicture = createThreePicture;
