// src/app/app.module.ts
import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { HttpClientModule } from '@angular/common/http';

import { AppComponent } from './app.component';
import { BlockListComponent } from './block-list/block-list.component';
import { BlockService } from "./block.service";

@NgModule({
  declarations: [
    AppComponent,
    BlockListComponent
  ],
  imports: [
    BrowserModule,
    HttpClientModule
  ],
  providers: [BlockService],
  bootstrap: [AppComponent]
})
export class AppModule { }
