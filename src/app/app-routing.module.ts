import {NgModule} from '@angular/core';
import {RouterModule, Routes} from '@angular/router';
import {GalleryComponent} from './gallery/gallery.component';
import {StudioComponent} from "./studio/studio.component";

const routes: Routes = [
  {path: '', component: GalleryComponent, pathMatch: 'full'},
  {path: 'gallery', component: GalleryComponent, pathMatch: 'full'},
  {path: 'studio', component: StudioComponent, pathMatch: 'full'},
  {path: 'gallery/:id', component: GalleryComponent},
  {path: 'studio/:id', component: StudioComponent, pathMatch: 'full'},
  {path: 'studio', component: StudioComponent},
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule {
}
