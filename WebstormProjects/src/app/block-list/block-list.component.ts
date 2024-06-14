import { Component, OnInit } from '@angular/core';
import { BlockService } from '../block.service';
// import { DomSanitizer, SafeUrl } from '@angular/platform-browser';

@Component({
  selector: 'app-block-list',
  templateUrl: './block-list.component.html',
  styleUrls: ['./block-list.component.css']
})
export class BlockListComponent implements OnInit {
  blocks: any[] = [];
  news: any[] =[];
  displayedColumns_block: string[] = ['height_block', 'fee_block', 'hash_block'];
  displayedColumns_news: string[] = ['id_news', 'title_news']

  constructor(private dataService: BlockService) { }

  ngOnInit(): void {
    this.dataService.getBlocks().subscribe(data => {
      this.blocks = data.slice(0,10);
    });
    this.dataService.getNews().subscribe(response => {
      this.news = response.slice(0,10);
    });
  }

}
