import { Component, OnInit } from '@angular/core';
import { BlockService } from '../block.service';

@Component({
  selector: 'app-block-list',
  templateUrl: './block-list.component.html',
  styleUrls: ['./block-list.component.css']
})
export class BlockListComponent implements OnInit {
  heights: any;

  constructor(private dataService: BlockService) { }

  ngOnInit(): void {
    this.dataService.getHeights().subscribe((response: any) => {
      this.heights = response;
    });
  }
}
