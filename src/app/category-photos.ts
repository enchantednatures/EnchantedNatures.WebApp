import { Photo } from "./photo";

export interface CategoryPhotos {
  id: number,
  name: string,
  photos: Photo[]
}
