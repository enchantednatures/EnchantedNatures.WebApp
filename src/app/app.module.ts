import { NgModule, OnInit, inject } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { GalleryComponent } from './gallery/gallery.component';
import { HttpClientModule } from '@angular/common/http';
import { NavMenuComponent } from './nav-menu/nav-menu.component';
import { HorizontalScrollDirective } from './horizontal-scroll.directive';

@NgModule({
  declarations: [
    AppComponent,
    GalleryComponent,
    NavMenuComponent,
    HorizontalScrollDirective
  ],
  imports: [
    HttpClientModule,
    BrowserModule,
    AppRoutingModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {

}

